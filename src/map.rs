use bevy::{prelude::*, render::camera};

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

#[derive(Component)]
pub struct MyGameCamera;

pub fn spawn_camera(mut commands: Commands) {
    let camera = camera::OrthographicProjection::default();

    commands.spawn((camera, MyGameCamera));
}