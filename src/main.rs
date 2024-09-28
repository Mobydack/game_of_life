mod components;
mod storages;
mod systems;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, systems::camera::setup)
        .add_systems(Update, systems::camera::update);

    app.run();
}
