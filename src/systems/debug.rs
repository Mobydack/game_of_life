use bevy::{diagnostic::Diagnostics, prelude::*};
use iyes_perf_ui::prelude::*;

use crate::components::{cell_bundle::Position, debug_entries::PerfUiCellsCount};
use crate::diagnostics::CELLS_COUNT;

pub fn setup_deubg(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot { ..default() },
        PerfUiEntryFPS::default(),
        PerfUiEntryFPSWorst::default(),
        PerfUiCellsCount::default(),
    ));
}

pub fn update_cells_diagnostic(query: Query<Entity, With<Position>>, mut diagnostics: Diagnostics) {
    diagnostics.add_measurement(&CELLS_COUNT, || query.iter().count() as f64);
}
