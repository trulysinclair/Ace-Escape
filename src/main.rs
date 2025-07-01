use bevy::color::palettes::tailwind::{SKY_700, SLATE_950};
use bevy::prelude::*;

fn main() {
    let background_color: Color = Color::from(SLATE_950);

    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(background_color))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(SKY_700))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}
