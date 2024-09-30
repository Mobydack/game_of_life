use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

use crate::components::cell_bundle::{CellBundle, Position};
use crate::events::cell::{InsertCellEvent, KillCellEvent};
use crate::resources::settings::SettingsResource;

pub fn init_cells(mut insert_cell_event: EventWriter<InsertCellEvent>) {
    insert_cell_event.send_batch([
        InsertCellEvent(Position::new(0, 1)),
        InsertCellEvent(Position::new(-1, 0)),
        InsertCellEvent(Position::new(0, 0)),
        InsertCellEvent(Position::new(1, 0)),
        InsertCellEvent(Position::new(-1, -1)),
        InsertCellEvent(Position::new(1, -1)),
    ]);
}

pub fn next_generation(
    mut insert_cell_event: EventWriter<InsertCellEvent>,
    mut kill_cell_event: EventWriter<KillCellEvent>,
    query: Query<(Entity, &Position)>,
) {
    let positions: Arc<Mutex<HashSet<Position>>> = Arc::new(Mutex::new(HashSet::new()));
    let neighbor_counts: Arc<Mutex<HashMap<Position, u32>>> = Arc::new(Mutex::new(HashMap::new()));
    let to_insert: Arc<Mutex<HashSet<InsertCellEvent>>> = Arc::new(Mutex::new(HashSet::new()));
    let to_kill: Arc<Mutex<HashSet<KillCellEvent>>> = Arc::new(Mutex::new(HashSet::new()));

    query.par_iter().for_each(|(_, position)| {
        let mut locked_positions = positions.lock().unwrap();
        let mut locked_neighbor_counts = neighbor_counts.lock().unwrap();

        locked_positions.insert(*position);

        for neighbor in position.neighbors() {
            let counts = locked_neighbor_counts.entry(neighbor).or_insert(0);

            *counts += 1;
        }
    });

    query.par_iter().for_each(|(entity, position)| {
        let mut locked_to_kill = to_kill.lock().unwrap();
        let locked_neighbors_count = neighbor_counts.lock().unwrap();
        let neighbor_count = locked_neighbors_count.get(position).unwrap_or(&0);

        if neighbor_count.lt(&2) || neighbor_count.gt(&3) {
            locked_to_kill.insert(KillCellEvent(entity));
        }
    });

    let locked_positions = positions.lock().unwrap();
    let mut locked_to_insert = to_insert.lock().unwrap();
    let locked_to_kill = to_kill.lock().unwrap();

    neighbor_counts
        .lock()
        .unwrap()
        .iter()
        .for_each(|(position, count)| {
            if !locked_positions.contains(position) && count.eq(&3) {
                locked_to_insert.insert(InsertCellEvent(*position));
            }
        });

    insert_cell_event.send_batch(locked_to_insert.iter().cloned());
    kill_cell_event.send_batch(locked_to_kill.iter().cloned());
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
        commands.entity(*entity).despawn();
    }
}
