use super::components::*;
use bevy::prelude::*;

pub fn movement(
    mut camera_query: Query<
        (
            &mut Transform,
            &GameCameraSettingsMovement,
            &GameCameraSettingsKeyBindings,
        ),
        (With<ActiveGameCamera>),
    >,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Real>>,
) {
    camera_query.par_iter_mut().for_each(
        |(mut transform, movement_settings, key_bindings_settings)| {
            let mut delta = Vec3::ZERO;

            if button_input.any_pressed(key_bindings_settings.left.iter().cloned()) {
                delta += Vec3::from(transform.left());
            }

            if button_input.any_pressed(key_bindings_settings.right.iter().cloned()) {
                delta += Vec3::from(transform.right());
            }

            if button_input.any_pressed(key_bindings_settings.up.iter().cloned()) {
                delta += Vec3::from(transform.up());
            }

            if button_input.any_pressed(key_bindings_settings.down.iter().cloned()) {
                delta += Vec3::from(transform.down());
            }

            transform.translation +=
                delta.normalize_or_zero() * time.delta_seconds() * movement_settings.speed;
        },
    );
}
