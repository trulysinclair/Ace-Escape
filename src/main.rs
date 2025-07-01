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

#[derive(Resource, Deref)]
struct CorruptionAudio(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    let corruption_sound = asset_server.load("sounds/corruption.ogg");
    let sound = corruption_sound.clone();
    commands.insert_resource(CorruptionAudio(corruption_sound));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(SKY_700))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));

    let t = commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
}
