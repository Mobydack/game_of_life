use bevy::prelude::*;

use crate::resources::settings::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

pub fn update(
    mut query: Query<&mut Transform, With<Camera2d>>,
    settings: Res<SettingsResource>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut camera) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta = direction.normalize_or_zero() * settings.camera.speed * time.delta_seconds();

    camera.translation += move_delta.extend(0.);
}
