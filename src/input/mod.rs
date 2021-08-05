use bevy::prelude::*;

pub mod local_player;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(local_player::Plugin);
    }
}
