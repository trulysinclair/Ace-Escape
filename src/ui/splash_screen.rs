use ace_escape::{despawn_screen, GameState};
use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct SplashScreenPlugin;

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>)
            .insert_resource(ClearColor(Color::BLACK));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/logo.png");

    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnSplashScreen,
        BackgroundColor(Color::BLACK),
        children![(
            ImageNode::new(icon),
            Node {
                width: Val::Px(200.0),
                ..default()
            }
        )],
    ));

    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)))
}

// Stay on splash, then move to the main menu.
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu)
    }
}
