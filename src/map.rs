use bevy::{prelude::*};
use bevy_ecs_tilemap::{map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType}, prelude::get_tilemap_center_transform, tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex}, TilemapBundle};

use crate::{assets::ImageAssets, player::Player};

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

  // Tile counts
  let map_size = TilemapSize { x: 60, y: 30 };

  // Where we will store the tilemap entity
  let tilemap_entity = commands.spawn_empty().id();

  
  let mut tile_storage = TileStorage::empty(map_size);

  for x in 0..map_size.x {
    for y in 0..map_size.y {
      let tile_pos = TilePos { x, y };
      let tile_entity = commands
        .spawn(TileBundle {
          position: tile_pos,
          tilemap_id: TilemapId(tilemap_entity),
          // Texture Atlas index
          texture_index: TileTextureIndex(14 as u32),
          ..Default::default()
        }
      )
      .id();

    tile_storage.set(&tile_pos, tile_entity);
        
    }
  }

  // Actual map size by a given tile count
  let tile_size = TilemapTileSize { x: 64., y: 64. };
  let grid_size = tile_size.into();
  let map_type = TilemapType::default();

  commands.entity(tilemap_entity).insert(TilemapBundle {
    grid_size,
    map_type,
    size: map_size,
    storage: tile_storage,
    texture: TilemapTexture::Single(my_assets.ground.clone()),
    tile_size,
    transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
    ..Default::default()
  });
}

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.;

/// Update the camera position by tracking the player.
pub fn update_camera(
  mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
  player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
  time: Res<Time>,
) {
  let Ok(mut camera) = camera.get_single_mut() else {
      return;
  };

  let Ok(player) = player.get_single() else {
      return;
  };

  let Vec3 { x, y, .. } = player.translation;
  let direction = Vec3::new(x, y, camera.translation.z);

  // Applies a smooth effect to camera movement using stable interpolation
  // between the camera position and the player position on the x and y axes.
  camera
      .translation = camera.translation.lerp(direction, time.delta_seconds() * CAMERA_DECAY_RATE) ;
}


#[derive(Component)]
pub struct MyGameCamera;

pub fn spawn_camera(mut commands: Commands) {    
  let camera =  Camera2dBundle {
    ..Default::default()
  };
  // camera.projection.scale *= 1.5;
    commands.spawn(camera);
}