use bevy::{prelude::*, tasks::prelude::*};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_system.system())
            .add_system(destruction_system.system());
    }
}

pub struct LocalPlayer;

pub struct HealthPoint {
    pub value: u32,
    pub max_value: u32,
}

pub struct MoveSpeed(pub f32);

pub struct MoveDirection(pub Vec2);

fn move_system(
    pool: Res<ComputeTaskPool>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MoveSpeed, &MoveDirection)>,
) {
    query.par_for_each_mut(&pool, 32, |(mut transform, move_speed, move_direction)| {
        transform.translation +=
            (move_direction.0 * move_speed.0 * time.delta_seconds()).extend(0.0);
    });
}

fn destruction_system(mut commands: Commands, query: Query<(Entity, &HealthPoint)>) {
    for (entity, health_point) in query.iter() {
        if health_point.value == 0 {
            commands.entity(entity).despawn();
        }
    }
}
