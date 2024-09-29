use bevy::prelude::*;

use crate::components::cell_bundle::Position;

#[derive(Event)]
pub struct InsertCellEvent(pub Position);

#[derive(Event)]
pub struct KillCellEvent(pub Entity);
