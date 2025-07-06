use bevy::app::Plugin;
use bevy::prelude::Component;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}

#[derive(Component)]
pub struct Player;
