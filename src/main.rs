mod enemy;
mod player;

use crate::{enemy::EnemyPlugin, player::PlayerPlugin};
use bevy::{color::palettes::tailwind::SLATE_950, prelude::*};

fn main() {
    let background_color: Color = Color::from(SLATE_950);

    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .insert_resource(ClearColor(background_color))
        .run();
}

#[derive(Component)]
struct Asteroid;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
