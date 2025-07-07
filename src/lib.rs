use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Game,
    GameMenu,
    GameOver,
}

#[derive(Component)]
pub struct Player;

pub fn reset_game() {}

pub fn start_game() {}

/**
 * Generic system for despawning all entities within a component.
 */
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn();
    }
}
