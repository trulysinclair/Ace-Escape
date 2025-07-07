mod enemy;
mod menu;
mod menu_example;
mod player;
mod splash_screen;

use crate::{enemy::EnemyPlugin, player::PlayerPlugin, splash_screen::SplashScreenPlugin};
use ace_escape::GameState;
use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::{color::palettes::tailwind::SLATE_950, prelude::*};

fn main() {
    let background_color: Color = Color::from(SLATE_950);

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
        .add_plugins((SplashScreenPlugin))
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .insert_resource(ClearColor(background_color))
        .init_state::<GameState>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
