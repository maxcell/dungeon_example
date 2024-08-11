use bevy::prelude::*;

use crate::assets::ImageAssets;

#[derive(Component)]
pub struct Player;


pub fn spawn_player(mut commands: Commands, my_assets: Res<ImageAssets>) {

  commands.spawn((SpriteBundle {
    texture: my_assets.player.clone(),
    // Spawn it on top of the layer of the map
    transform: Transform::from_xyz(0., 0., 1.),
    sprite: Sprite {
      ..Default::default()
    },
    ..Default::default()
   }, Player));
}

// TODO: Come and tidy this up a bit so not just on keyboard
pub fn move_player(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
  let mut player_transform = query.single_mut();

  // Direction, tile speed, and scaling it by consistent frame-rate
  if keyboard_input.pressed(KeyCode::ArrowLeft) ||  keyboard_input.pressed(KeyCode::KeyA) {
    player_transform.translation.x += ( -1.0) * 64.0 * time.delta_seconds();
  }
  if keyboard_input.pressed(KeyCode::ArrowRight) ||  keyboard_input.pressed(KeyCode::KeyD) {
    player_transform.translation.x += ( 1.0) * 64.0 * time.delta_seconds();
  }

  if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
    player_transform.translation.y += ( 1.0) * 64.0 * time.delta_seconds();
  }

  if keyboard_input.pressed(KeyCode::ArrowDown) ||  keyboard_input.pressed(KeyCode::KeyS) {
    player_transform.translation.y += (-1.0) * 64.0 * time.delta_seconds();
  }
}