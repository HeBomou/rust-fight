use bevy::prelude::*;

pub mod hull;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(hull::Plugin);
    }
}
