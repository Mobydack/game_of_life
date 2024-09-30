use bevy::prelude::*;

use crate::resources::settings::SettingsResource;

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn neighbors(self) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(8);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    neighbors.push(Position {
                        x: self.x + dx,
                        y: self.y + dy,
                    });
                }
            }
        }

        neighbors
    }
}

#[derive(Bundle)]
pub struct CellBundle {
    pub position: Position,
    pub sprite: SpriteBundle,
}

impl CellBundle {
    pub fn new(position: Position, settings: &SettingsResource) -> Self {
        let translation = Vec3::new(
            position.x.clone() as f32 * (settings.cell.size + settings.cell.padding),
            position.y as f32 * (settings.cell.size + settings.cell.padding),
            0.,
        );

        Self {
            position,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(settings.cell.size, settings.cell.size)),
                    ..default()
                },
                transform: Transform::from_translation(translation),
                ..default()
            },
        }
    }
}
