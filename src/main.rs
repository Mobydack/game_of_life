mod components;
mod events;
mod resources;
mod storages;
mod systems;

use bevy::prelude::*;
use resources::settings::SettingsResource;

fn main() {
    let mut app = App::new();

    app.insert_resource(SettingsResource { ..default() })
        .insert_resource(Time::<Fixed>::from_seconds(0.2));

    app.add_plugins(DefaultPlugins);

    app.add_event::<events::cell::InsertCellEvent>()
        .add_event::<events::cell::KillCellEvent>();

    app.add_systems(
        Startup,
        (systems::camera::setup, systems::cell::init_cells).chain(),
    )
    .add_systems(
        Update,
        (
            systems::camera::update,
            systems::cell::insert_cell_listener,
            systems::cell::kill_cell_listener,
        ),
    )
    .add_systems(FixedUpdate, systems::cell::next_generation);

    app.run();
}
