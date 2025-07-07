use bevy::prelude::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Component)]
struct Asteroid;

pub fn asteroid_laser_collision() {}

pub fn player_asteroid_collision() {}
