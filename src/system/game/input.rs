use crate::command::CursorLock;
use bevy::app::AppExit;
use bevy::prelude::Commands;
use bevy::prelude::EventWriter;
use bevy::prelude::Input;
use bevy::prelude::KeyCode;
use bevy::prelude::Res;

pub fn input(
    mut commands: Commands,
    mut events: EventWriter<AppExit>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        events.send(AppExit);
    }

    if keyboard.just_pressed(KeyCode::Tab) {
        commands.add(CursorLock(false));
    }

    if keyboard.just_pressed(KeyCode::O) {}
}
