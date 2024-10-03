use bevy::prelude::*;

pub struct CellSettings {
    pub size: f32,
    pub padding: f32,
}

#[derive(Resource)]
pub struct SettingsResource {
    pub cell: CellSettings,
}

impl Default for SettingsResource {
    fn default() -> Self {
        Self {
            cell: CellSettings {
                size: 10.,
                padding: 2.,
            },
        }
    }
}
