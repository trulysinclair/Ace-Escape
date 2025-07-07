mod asteroid;

use bevy::prelude::{Component, States};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Component)]
pub struct Player;

pub fn reset_game() {}

pub fn start_game() {}
