use crate::player::Player;
use crate::CorruptionSound;
use bevy::app::Startup;
use bevy::asset::{AssetServer, Assets};
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::color::palettes::tailwind::RED_700;
use bevy::color::Color;
use bevy::prelude::{
    Circle, ColorMaterial, Commands, Component, Local, Mesh, Mesh2d, MeshMaterial2d, Plugin, Query,
    Res, ResMut, Transform, With,
};
use bevy::time::Time;
use std::time::Duration;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct Deimos;

#[derive(Component)]
pub struct DeimosOwned;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Emitter
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(Color::from(RED_700))),
        Transform::default(),
        AudioPlayer::new(asset_server.load("sounds/corruption.ogg")),
        PlaybackSettings::LOOP.with_spatial(true),
        CorruptionSound,
    ));
}

fn deimos_movement(mut query: Query<&mut Transform, With<Deimos>>, time: Res<Time>) {}

fn deimos_weapon_system(
    mut commands: Commands,
    time: Res<Time>,
    query_player: Query<&Transform, With<Player>>,
    query_deimos: Query<&Transform, With<Deimos>>,
    mut last_shot: Local<Option<Duration>>,
) {
}
