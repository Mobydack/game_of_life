mod components;
mod events;
mod resources;
mod storages;
mod systems;

use bevy::prelude::*;
use resources::settings::SettingsResource;

fn main() {
    let mut app = App::new();

    app.insert_resource(SettingsResource { ..default() });
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, systems::camera::setup)
        .add_systems(Update, systems::camera::update);

    app.run();
}
