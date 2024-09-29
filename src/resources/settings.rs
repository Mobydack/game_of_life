use bevy::prelude::*;

pub struct CameraSettings {
    pub speed: f32,
}

pub struct CellSettings {
    pub size: f32,
    pub padding: f32,
}

#[derive(Resource)]
pub struct SettingsResource {
    pub camera: CameraSettings,
    pub cell: CellSettings,
}

impl Default for SettingsResource {
    fn default() -> Self {
        Self {
            camera: CameraSettings { speed: 100. },
            cell: CellSettings {
                size: 10.,
                padding: 2.,
            },
        }
    }
}
