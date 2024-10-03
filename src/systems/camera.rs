use crate::plugins::camera::components::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        GameCamera,
        ActiveGameCamera,
        GameCameraSettingsMovement::default(),
        GameCameraSettingsKeyBindings::default(),
    ));
}
