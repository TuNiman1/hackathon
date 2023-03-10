use crate::command::ActorActionsSet;
use crate::command::ActorDirectionSet;
use crate::command::ActorPlayerSet;
use crate::command::ActorSet;
use crate::command::ClientJoin;
use crate::command::EntityDelete;
use crate::command::ProjectileSpawn;
use crate::command::Start;
use crate::component::ActorActions;
use crate::resource::EntityConverter;
use crate::resource::Message;
use crate::resource::NetConnection;
use crate::resource::NetResource;
use crate::resource::PositionUpdateResource;
use crate::resource::MESSAGE_SIZE_MAX;
use bevy::ecs::entity::Entities;
use bevy::ecs::entity::Entity;
use bevy::prelude::Commands;
use bevy::prelude::ResMut;
use std::io::ErrorKind;
use std::net::SocketAddr;

pub fn message_receive(
    entities: &Entities,
    mut commands: Commands,
    mut entity_converter: ResMut<EntityConverter>,
    mut position_updates: ResMut<PositionUpdateResource>,
    mut net: ResMut<NetResource>,
) {
    let is_server = net.is_server();

    let mut responses = Vec::new();

    loop {
        let mut buffer = [0; MESSAGE_SIZE_MAX];

        let (message_length, address) = match net.socket.recv_from(&mut buffer) {
            Ok((message_length, address)) => (message_length, address),
            Err(error) => {
                if error.kind() == ErrorKind::WouldBlock {
                    break;
                } else {
                    log::warn!("Failed to receive a message: {:?}", error);
                    continue;
                }
            }
        };

        let message = match Message::decode(&buffer[..message_length]) {
            Ok(message) => message,
            Err(error) => {
                log::warn!("A corrupted message received from {}: {:?}", address, error);
                continue;
            }
        };

        let connection = net.connections.entry(address).or_insert_with(|| {
            log::info!("{} connected", address);
            return NetConnection::new();
        });

        if let Message::Response { message_id } = message {
            connection.acknowledge_message(message_id);
        } else {
            if let Some(message_id) = message.get_id() {
                responses.push((address, Message::Response { message_id }));
            }

            if let Some(message) = connection.filter_message(message) {
                let entity = connection.attached_entity;
                let next_messages = connection.take_next_held_messages();

                on_message(
                    &address,
                    &message,
                    entity,
                    &entities,
                    &mut entity_converter,
                    &mut commands,
                    &mut position_updates,
                    is_server,
                );

                for message in &next_messages {
                    on_message(
                        &address,
                        message,
                        entity,
                        &entities,
                        &mut entity_converter,
                        &mut commands,
                        &mut position_updates,
                        is_server,
                    );
                }
            }
        }
    }

    for (address, message) in responses {
        net.send_unreliably_to(&address, &message);
    }
}

fn on_message(
    address: &SocketAddr,
    message: &Message,
    entity: Option<Entity>,
    entities: &Entities,
    converter: &mut EntityConverter,
    commands: &mut Commands,
    position_updates: &mut PositionUpdateResource,
    is_server: bool,
) {
    if is_server {
        on_message_as_server(address, message, entity, commands);
    } else {
        on_message_as_client(message, entities, converter, commands, position_updates);
    }
}

fn on_message_as_server(
    address: &SocketAddr,
    message: &Message,
    entity: Option<Entity>,
    commands: &mut Commands,
) {
    match *message {
        Message::Join { .. } => {
            commands.add(ClientJoin(*address));
        }
        Message::ClientInput {
            actions, direction, ..
        } => {
            if let Some(entity) = entity {
                commands.add(ActorActionsSet {
                    entity,
                    actions: ActorActions::from_bits_truncate(actions),
                    direction,
                });
            }
        }
        Message::ClientInputDirection { direction, .. } => {
            if let Some(entity) = entity {
                commands.add(ActorDirectionSet { entity, direction });
            }
        }
        _ => {}
    }
}

fn on_message_as_client(
    message: &Message,
    entities: &Entities,
    converter: &mut EntityConverter,
    commands: &mut Commands,
    position_updates: &mut PositionUpdateResource,
) {
    match *message {
        Message::JoinAccept { .. } => {
            commands.add(Start);
        }
        Message::ActorSpawn {
            entity_id,
            actor_type,
            position,
            ..
        } => {
            commands.add(ActorSet {
                entity: converter.to_internal(entities, entity_id),
                config: actor_type.into(),
                position,
                is_ghost: false,
            });
        }
        Message::ActorGrant { entity_id, .. } => {
            commands.add(ActorPlayerSet(converter.to_internal(entities, entity_id)));
        }
        Message::PositionUpdate {
            entity_id,
            position,
        } => {
            position_updates.push((converter.to_internal(entities, entity_id).id(), position));
        }
        Message::ProjectileSpawn {
            position,
            velocity,
            acceleration_factor,
            shooter_id,
            ..
        } => {
            commands.add(ProjectileSpawn {
                position,
                velocity,
                acceleration_factor,
                shooter: shooter_id.map(|id| converter.to_internal(entities, id)),
            });
        }
        Message::EntityDelete { entity_id, .. } => {
            commands.add(EntityDelete(converter.to_internal(entities, entity_id)));
        }
        _ => {}
    }
}
