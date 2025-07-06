use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    color::palettes::tailwind::SKY_700,
    color::Color,
    input::ButtonInput,
    math::Vec3,
    prelude::*,
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, movement);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let listener = SpatialListener::new(400.);

    // Player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::from(SKY_700))),
        listener.clone(),
        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
        Visibility::default(),
    ));
}

fn movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut listener: Single<&mut Transform, With<SpatialListener>>,
) {
    let speed = 360.;

    if keyboard.pressed(KeyCode::ArrowRight) {
        listener.translation.x += speed * time.delta_secs()
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        listener.translation.x -= speed * time.delta_secs()
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        listener.translation.y += speed * time.delta_secs()
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        listener.translation.y -= speed * time.delta_secs()
    }
}
