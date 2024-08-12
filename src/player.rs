use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, InputManagerBundle};

use crate::{assets::ImageAssets, controls::PlayerAction};

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
   }, InputManagerBundle::with_map(
    PlayerAction::default_input_map()
   ))).insert(Player);
}

pub fn move_player(query: Query<&ActionState<PlayerAction>, With<Player>>, mut player_query:  Query<&mut Transform, With<Player>>, time: Res<Time>) {
  let action_state = query.single();
  let mut player_transform = player_query.single_mut();

  let axis_pair = action_state.axis_pair(&PlayerAction::Move);


  player_transform.translation.x += (axis_pair.x) * 64.0 * time.delta_seconds();
  player_transform.translation.y += (axis_pair.y) * 64.0 * time.delta_seconds();
}