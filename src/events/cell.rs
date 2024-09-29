use bevy::prelude::*;

use crate::components::cell_bundle::Position;

#[derive(Event)]
pub struct InsertCellEvent(Position);

#[derive(Event)]
pub struct KillCellEvent(Entity);
