use bevy::{prelude::*};

use crate::assets::ImageAssets;

const TILE_SIZE: f32 = 64.0;
const TILE_SPACER: f32 = 0.0;


#[derive(Component)]
pub struct Map {
  pub size: u16, // 2_16
  pub physical_size: f32
}

impl Map {
  fn new(size: u16) -> Self {
    let physical_size = f32::from(size) * TILE_SIZE  + f32::from(size + 1) * TILE_SPACER;

    Map {
      size,
      physical_size
    }
  }
}

pub fn spawn_world(mut commands: Commands, my_assets: Res<ImageAssets>) {
  let map = Map::new(30);

  let mut texture_atlas = TextureAtlas::from(my_assets.layout.clone());
  texture_atlas.index = 14;

  commands.insert()
  
  // commands.spawn((SpriteBundle {
  //     texture: my_assets.ground.clone(),
  //     // transform: Transform::from_xyz(0.0, -64.0, 0.0),
  //   ..default()
  // }, texture_atlas));
}


#[derive(Component)]
pub struct MyGameCamera;

pub fn spawn_camera(mut commands: Commands) {    
    commands.spawn(Camera2dBundle::default());
}