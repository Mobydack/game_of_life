use bevy::prelude::*;

use crate::components::cell_bundle::Position;

#[derive(Event, Clone, PartialEq, Eq, Hash)]
pub struct InsertCellEvent(pub Position);

#[derive(Event, Clone, PartialEq, Eq, Hash)]
pub struct KillCellEvent(pub Entity);
