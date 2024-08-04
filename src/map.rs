use bevy::{prelude::*};

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

pub fn spawn_world(mut commands: Commands) {
  let map = Map::new(30);
  
  commands.spawn(SpriteBundle {
      sprite: Sprite {
          
          color: Color::linear_rgb(100.0, 0.0, 0.0),
          custom_size: Some(Vec2::new(map.physical_size, map.physical_size)),
          ..default()
      }
      ,..default()
  });
}


#[derive(Component)]
pub struct MyGameCamera;

pub fn spawn_camera(mut commands: Commands) {    
    commands.spawn(Camera2dBundle::default());
}