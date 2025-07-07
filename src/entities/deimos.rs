use crate::entities::player::Player;
use crate::game::OnGameScreen;
use ace_escape::GameState;
use bevy::prelude::*;
use std::time::Duration;

pub struct DeimosPlugin;

#[derive(Component)]
pub struct CorruptionSound;

impl Plugin for DeimosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, (update_speed, pause, movement));
    }
}

#[derive(Component)]
pub struct Deimos {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Resource)]
struct DecayRate(f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/deimos.png");

    // Emitter
    commands.spawn((
        Sprite {
            image: ship_handle,
            texture_atlas: None,
            color: Default::default(),
            flip_x: false,
            flip_y: true,
            custom_size: None,
            rect: None,
            anchor: Default::default(),
            image_mode: Default::default(),
        },
        AudioPlayer::new(asset_server.load("sounds/corruption.ogg")),
        PlaybackSettings::LOOP.with_spatial(true),
        CorruptionSound,
        Deimos {
            movement_speed: 1.0,
            rotation_speed: f32::to_radians(360.0),
        },
        OnGameScreen,
    ));

    commands.insert_resource(DecayRate(1.0));
}

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

fn movement(
    deimos: Single<(&Deimos, &mut Transform), With<Deimos>>,
    player: Single<&Transform, (With<Player>, Without<Deimos>)>,
    decay_rate: Res<DecayRate>,
    time: Res<Time>,
) {
    let player_translation = player.translation.xy();
    let decay_rate = decay_rate.0;

    let (deimos, mut deimos_transform) = deimos.into_inner();
    let forward = (deimos_transform.rotation * Vec3::Y).xy();
    let to_player = (player_translation - deimos_transform.translation.xy()).normalize();
    let forward_dot_player = forward.dot(to_player);

    if (forward_dot_player - 1.0).abs() < f32::EPSILON {
        return;
    }

    let right = (deimos_transform.rotation * Vec3::X).xy();
    let right_dot_player = right.dot(to_player);
    let rotation_sign = -f32::copysign(1.0, right_dot_player);
    let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0));

    let angle = rotation_sign * (deimos.rotation_speed * time.delta_secs()).min(max_angle);

    deimos_transform.rotate_z(angle);
    deimos_transform.translation.smooth_nudge(
        &player.translation,
        decay_rate,
        deimos.movement_speed * time.delta_secs(),
    );
}

pub fn deimos_laser_collision() {}
