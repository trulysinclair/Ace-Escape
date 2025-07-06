mod enemy;
mod player;

use crate::enemy::EnemyPlugin;
use bevy::{
    color::palettes::tailwind::{SKY_700, SLATE_950},
    prelude::*,
};

fn main() {
    let background_color: Color = Color::from(SLATE_950);

    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (update_speed, pause))
        .add_plugins((DefaultPlugins, EnemyPlugin))
        .insert_resource(ClearColor(background_color))
        .run();
}

#[derive(Component)]
struct CorruptionSound;

#[derive(Component)]
struct Asteroid;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::from(SKY_700))),
        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
        Visibility::default(),
    ));
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
