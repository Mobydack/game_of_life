mod components;
mod diagnostics;
mod events;
mod plugins;
mod resources;
mod states;
mod storages;
mod systems;

use bevy::{
    diagnostic::{Diagnostic, FrameTimeDiagnosticsPlugin, RegisterDiagnostic},
    prelude::*,
};
use iyes_perf_ui::prelude::*;
use resources::settings::SettingsResource;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(plugins::camera::CameraPlugin);

    app.register_diagnostic(Diagnostic::new(diagnostics::CELLS_COUNT).with_suffix("game"));

    app.init_state::<states::GamePauseStates>();

    app.insert_resource(SettingsResource { ..default() })
        .insert_resource(Time::<Fixed>::from_seconds(0.1));

    app.add_event::<events::cell::InsertCellEvent>()
        .add_event::<events::cell::KillCellEvent>();

    app.add_systems(
        Startup,
        (
            (systems::camera::setup, systems::cell::init_cells).chain(),
            systems::debug::setup_deubg,
        ),
    )
    .add_systems(
        Update,
        (
            systems::cell::insert_cell_listener,
            systems::cell::kill_cell_listener,
            systems::debug::update_cells_diagnostic,
            systems::game::game_states_controller,
        ),
    )
    .add_systems(
        FixedUpdate,
        (systems::cell::next_generation).run_if(in_state(states::GamePauseStates::Running)),
    );

    app.add_perf_ui_simple_entry::<components::debug_entries::PerfUiCellsCount>();

    app.run();
}
