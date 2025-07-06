use bevy::prelude::{Component, States};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Menu,
    Playing,
}

#[derive(Component)]
pub struct Player;

pub fn reset_game() {}

pub fn start_game() {}

pub fn asteroid_laser_collision() {}

pub fn deimos_laser_collision() {}

pub fn player_asteroid_collision() {}
