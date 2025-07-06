use crate::player::Player;
use bevy::prelude::{Commands, Component, Local, Plugin, Query, Res, Transform, With};
use bevy::time::Time;
use std::time::Duration;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}

#[derive(Component)]
pub struct Deimos;

#[derive(Component)]
pub struct DeimosOwned;

fn deimos_movement(mut query: Query<&mut Transform, With<Deimos>>, time: Res<Time>) {}

fn deimos_weapon_system(
    mut commands: Commands,
    time: Res<Time>,
    query_player: Query<&Transform, With<Player>>,
    query_deimos: Query<&Transform, With<Deimos>>,
    mut last_shot: Local<Option<Duration>>,
) {
}
