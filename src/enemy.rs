use crate::player::Player;
use bevy::{
    app::{Startup, Update},
    asset::{AssetServer, Assets},
    audio::{AudioPlayer, AudioSink, AudioSinkPlayback, PlaybackSettings},
    color::palettes::tailwind::RED_700,
    color::Color,
    input::ButtonInput,
    math::ops,
    prelude::{
        Circle, ColorMaterial, Commands, Component, KeyCode, Local, Mesh, Mesh2d, MeshMaterial2d,
        Plugin, Query, Res, ResMut, Transform, With,
    },
    time::Time,
};
use std::time::Duration;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct CorruptionSound;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_speed, pause));
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

fn update_speed(music_controller: Query<&AudioSink, With<CorruptionSound>>, time: Res<Time>) {
    let Ok(sink) = music_controller.single() else {
        return;
    };

    sink.set_speed((ops::sin(time.elapsed_secs() / 5.0) + 1.0).max(0.1))
}

fn pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<CorruptionSound>>,
) {
    let Ok(sink) = music_controller.single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        sink.toggle_playback();
    }
}
