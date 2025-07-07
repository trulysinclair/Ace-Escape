use crate::game::OnGameScreen;
use ace_escape::GameState;
use bevy::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(FixedUpdate, movement)
            .insert_resource(Time::<Fixed>::from_hz(60.0));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let listener = SpatialListener::new(100.);
    let ship_handle = asset_server.load("textures/player.png");

    commands.spawn((
        Sprite {
            image: ship_handle,
            texture_atlas: None,
            color: Default::default(),
            flip_x: false,
            flip_y: true,
            custom_size: Option::from(Vec2::new(77., 41.)),
            rect: None,
            anchor: Default::default(),
            image_mode: Default::default(),
        },
        listener.clone(),
        Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
        },
        OnGameScreen,
    ));
}

fn movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Single<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.into_inner();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        movement_factor += 1.0;
    }

    // Update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_secs());

    // Get the ship's forward vector by applying the current rotation to the ships initial facing
    // vector
    let movement_direction = transform.rotation * Vec3::Y;
    // Get the distance the ship will move based on direction, the ship's movement speed and delta
    // time
    let movement_distance = movement_factor * player.movement_speed * time.delta_secs();
    // Create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // Update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // Bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}
