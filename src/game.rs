use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use ace_escape::{despawn_screen, GameState};
use bevy::color::palettes::tailwind::SLATE_950;
use bevy::prelude::*;

pub struct GamePlugin;

#[derive(Component)]
pub struct OnGameScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let background_color: Color = Color::from(SLATE_950);

        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                Update,
                update.run_if(in_state(GameState::Game).or(in_state(GameState::GameOver))),
            )
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .add_plugins((PlayerPlugin, EnemyPlugin))
            .insert_resource(ClearColor(background_color));
    }
}

fn setup(mut commands: Commands) {
    println!("Game setup");
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
}
