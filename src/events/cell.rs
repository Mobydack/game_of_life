use bevy::prelude::*;

use crate::components::cell_bundle::Position;

#[derive(Event, Clone, Copy)]
pub struct InsertCellEvent(pub Position);

#[derive(Event, Clone, Copy)]
pub struct KillCellEvent(pub Entity);
