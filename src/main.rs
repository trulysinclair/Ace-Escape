mod asteroids;
mod enemy;
mod game;
mod menu;
mod menu_example;
mod player;
mod splash_screen;

use crate::game::GamePlugin;
use crate::splash_screen::SplashScreenPlugin;
use ace_escape::GameState;
use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(
            DefaultPlugins
                .set(AudioPlugin {
                    default_spatial_scale: SpatialScale::new_2d(1. / 100.0),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ace Escape".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((SplashScreenPlugin, GamePlugin))
        .init_state::<GameState>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
