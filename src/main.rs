use bevy::color::palettes::tailwind::{SKY_700, SLATE_950};
use bevy::prelude::*;

fn main() {
    let background_color: Color = Color::from(SLATE_950);

    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (update_speed, pause))
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(background_color))
        .run();
}

// #[derive(Resource, Deref)]
// struct CorruptionAudio(Handle<AudioSource>);

#[derive(Component)]
struct CorruptionSound;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(SKY_700))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/corruption.ogg")),
        CorruptionSound,
    ));

    // let corruption_sound = asset_server.load("sounds/corruption.ogg");
    // let sound = corruption_sound.clone();
    // commands.insert_resource(CorruptionAudio(corruption_sound));
    // let t = commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
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
