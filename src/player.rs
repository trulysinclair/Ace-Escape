use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    color::palettes::tailwind::SKY_700,
    color::Color,
    math::Vec3,
    prelude::{
        ColorMaterial, Commands, Component, Mesh, Mesh2d, MeshMaterial2d, Rectangle, ResMut,
        Transform, Visibility,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::from(SKY_700))),
        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
        Visibility::default(),
    ));
}

#[derive(Component)]
pub struct Player;
