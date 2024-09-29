use std::ops::Not;
use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::components::cell_bundle::{CellBundle, Position};
use crate::events::cell::{InsertCellEvent, KillCellEvent};
use crate::resources::settings::SettingsResource;

pub fn init_cells(mut insert_cell_event: EventWriter<InsertCellEvent>) {
    insert_cell_event.send_batch([
        InsertCellEvent(Position::new(0, 0)),
        InsertCellEvent(Position::new(-1, 0)),
        InsertCellEvent(Position::new(1, 0)),
    ]);
}

pub fn next_generation(
    query: Query<(&Position, Entity), With<Position>>,
    mut insert_cell_event: EventWriter<InsertCellEvent>,
    mut kill_cell_event: EventWriter<KillCellEvent>,
) {
    let insert_cells_bundle: Arc<Mutex<Vec<InsertCellEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let kill_cells_bundle: Arc<Mutex<Vec<KillCellEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let positions = HashSet::from_iter(
        query
            .iter()
            .map(|(position, _)| format!("{}_{}", position.x, position.y)),
    );

    query.par_iter().for_each(|(position, entity)| {
        let neighbors_count = position
            .neighbors()
            .iter()
            .filter(|neighbor| positions.contains(&format!("{}_{}", neighbor.x, neighbor.y)))
            .count();

        match neighbors_count {
            ..1 | 4.. => {
                let mut storage = kill_cells_bundle.lock().unwrap();

                storage.push(KillCellEvent(entity));
            }
            _ => {
                let mut storage = insert_cells_bundle.lock().unwrap();

                position
                    .neighbors()
                    .iter()
                    .filter(|neighbor| {
                        positions
                            .contains(&format!("{}_{}", neighbor.x, neighbor.y))
                            .not()
                            && neighbor
                                .neighbors()
                                .iter()
                                .filter(|position| {
                                    positions.contains(&format!("{}_{}", position.x, position.y))
                                })
                                .count()
                                == 3
                    })
                    .for_each(|position| storage.push(InsertCellEvent(*position)));
            }
        }
    });

    insert_cell_event.send_batch(insert_cells_bundle.lock().unwrap().iter().cloned());
    kill_cell_event.send_batch(kill_cells_bundle.lock().unwrap().iter().cloned());
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

pub fn kill_cell_listener(mut kill_cell_event: EventReader<KillCellEvent>, mut commands: Commands) {
    for KillCellEvent(entity) in kill_cell_event.read() {
        commands.entity(*entity).despawn_recursive();
    }
}
