mod entities;
mod game;
mod menu_example;
mod ui;

use crate::game::GamePlugin;
use ace_escape::GameState;
use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::prelude::*;
use ui::splash_screen::SplashScreenPlugin;

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
