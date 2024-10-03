use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct GameCamera;

#[derive(Component, Debug, Clone, Copy)]
pub struct ActiveGameCamera;

#[derive(Component, Debug, Clone)]
pub struct GameCameraSettingsKeyBindings {
    pub up: Vec<KeyCode>,
    pub down: Vec<KeyCode>,
    pub left: Vec<KeyCode>,
    pub right: Vec<KeyCode>,
}

impl Default for GameCameraSettingsKeyBindings {
    fn default() -> Self {
        Self {
            up: vec![KeyCode::KeyW, KeyCode::ArrowUp],
            down: vec![KeyCode::KeyS, KeyCode::ArrowDown],
            left: vec![KeyCode::KeyA, KeyCode::ArrowLeft],
            right: vec![KeyCode::KeyD, KeyCode::ArrowRight],
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct GameCameraSettingsMovement {
    pub speed: f32,
}

impl Default for GameCameraSettingsMovement {
    fn default() -> Self {
        Self { speed: 250. }
    }
}
