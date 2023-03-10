use crate::component::Collision;
use crate::component::Projectile;
use crate::model::geometry::GeometryProjection;
use crate::util::ext::Vec2Ext;
use crate::util::math;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Query;
use bevy::math::Quat;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Commands;
use bevy::prelude::Res;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::Vec2;
use bevy::prelude::Without;
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

const TIME_DELTA_FOR_RENDER: Duration = Duration::from_millis(25); // 40 FPS
const SIZE: f32 = 0.6;

pub fn projectile(
    mut projectiles: Query<(Entity, &mut Projectile, &mut Transform)>,
    obstacles: Query<(Entity, &Collision, &Transform), Without<Projectile>>,
    mut commands: Commands,
    time: Res<Time>,
) -> Vec<(Entity, Vec2, f32)> {
    let mut hits = Vec::new();
    let t0 = time.time_since_startup();
    let t1 = t0.saturating_sub(time.delta());
    let t2 = t0.saturating_sub(Duration::max(time.delta(), TIME_DELTA_FOR_RENDER));

    for (entity, mut projectile, mut transform) in projectiles.iter_mut() {
        if projectile.stopped {
            commands.entity(entity).despawn();
            continue;
        }

        let (mut head, head_velocity) = projectile.calc_data(t0);
        let (tail, tail_velocity) = projectile.calc_data(t1);
        let tail_visual = projectile.calc_data(t2).0;

        if let Some((obstacle, obstacle_position, contact_position, _)) =
            find_obstacle(&(head, tail), projectile.shooter, &obstacles)
        {
            let contact_velocity =
                find_contact_velocity(contact_position, head, tail, head_velocity, tail_velocity);

            let angle =
                math::angle_difference(tail.atan2_to(head), tail.atan2_to(obstacle_position));

            hits.push((obstacle, contact_velocity * Projectile::MASS, angle));
            head = contact_position;
            projectile.stopped = true;
        }

        update_transform(&projectile, head, tail_visual, &mut transform);

        if has_stopped(head_velocity) {
            projectile.stopped = true;
        }
    }

    return hits;
}

fn find_obstacle(
    projectaile: &(Vec2, Vec2),
    shooter: Option<Entity>,
    obstacles: &Query<(Entity, &Collision, &Transform), Without<Projectile>>,
) -> Option<(Entity, Vec2, Vec2, f32)> {
    let mut result: Option<(Entity, Vec2, Vec2, f32)> = None;

    for (entity, collision, transform) in obstacles.iter() {
        if shooter == Some(entity) {
            continue;
        }

        let obstacle = transform.translation.xy();
        let contact = obstacle.project_on(projectaile);
        let contact_distance = obstacle.distance_squared(contact);

        if collision.radius * collision.radius > contact_distance {
            let tail_distance = obstacle.distance_squared(projectaile.1);

            if result.map_or(true, |o| o.3 > tail_distance) {
                result = Some((entity, obstacle, contact, tail_distance));
            }
        }
    }

    return result;
}

fn find_contact_velocity(
    contact: Vec2,
    head: Vec2,
    tail: Vec2,
    head_velocity: Vec2,
    tail_velocity: Vec2,
) -> Vec2 {
    let d0 = contact.distance(tail);
    let d1 = contact.distance(head);
    return tail_velocity - d0 / (d0 + d1) * (tail_velocity - head_velocity);
}

fn update_transform(projectile: &Projectile, head: Vec2, tail: Vec2, transform: &mut Transform) {
    let center = (head + tail) / 2.0;
    transform.translation.x = center.x;
    transform.translation.y = center.y;
    transform.rotation = Quat::from_rotation_z(projectile.initial_velocity.atan2() - FRAC_PI_2);
    transform.scale.x = SIZE;
    transform.scale.y = (head - tail).length();
}

fn has_stopped(velocity: Vec2) -> bool {
    return velocity.is_shorter_than(Projectile::VELOCITY_MIN);
}
