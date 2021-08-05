use bevy::prelude::*;

mod input;
mod tank;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(input::Plugin)
        .add_plugin(tank::Plugin)
        .run();
}
