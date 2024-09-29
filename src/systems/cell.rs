use bevy::prelude::*;

use crate::components::cell_bundle::{CellBundle, Position};
use crate::events::cell::InsertCellEvent;
use crate::resources::settings::SettingsResource;

pub fn init_cells(mut insert_cell_event: EventWriter<InsertCellEvent>) {
    insert_cell_event.send_batch([
        InsertCellEvent(Position::new(0, 0)),
        InsertCellEvent(Position::new(-1, 0)),
        InsertCellEvent(Position::new(1, 0)),
    ]);
}

pub fn insert_cell_listener(
    mut insert_cell_event: EventReader<InsertCellEvent>,
    mut commands: Commands,
    settings: Res<SettingsResource>,
) {
    let mut bundles_to_insert: Vec<CellBundle> = Vec::new();

    for InsertCellEvent(position) in insert_cell_event.read() {
        bundles_to_insert.push(CellBundle::new(*position, &settings));
    }

    commands.spawn_batch(bundles_to_insert);
}
