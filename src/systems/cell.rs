use bevy::prelude::*;

use crate::events::cell::InsertCellEvent;

pub mod event_listeners {
    use bevy::prelude::*;

    use crate::events::cell::InsertCellEvent;

    pub fn insert_cell(mut insert_cell_event: EventReader<InsertCellEvent>) {}
}
