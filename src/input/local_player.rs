use bevy::prelude::*;

use crate::tank::hull;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(move_system.system());
    }
}

fn move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut hull::MoveDirection, With<hull::LocalPlayer>>,
) {
    let mut move_dir = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        move_dir += Vec2::new(-1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_dir += Vec2::new(1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::W) {
        move_dir += Vec2::new(0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_dir += Vec2::new(0.0, -1.0);
    }
    if let Ok(mut move_direction) = query.single_mut() {
        move_direction.0 = move_dir;
    }
}
