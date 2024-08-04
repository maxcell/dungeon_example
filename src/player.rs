use bevy::prelude::{Commands, Component};

#[derive(Component)]
pub struct Player;


pub fn spawn_player(mut commands: Commands) {
  commands.spawn(Player);
}