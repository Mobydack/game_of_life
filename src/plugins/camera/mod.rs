pub mod components;
mod systems;

use bevy::prelude::*;

#[derive(SystemSet, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CameraSystemsSet;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (systems::movement).in_set(CameraSystemsSet));
    }
}
