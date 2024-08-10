use bevy::prelude::*;
use bevy_asset_loader::asset_collection::{AssetCollection, AssetCollectionApp};


pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
  fn build(&self, app: &mut App) {
    app.init_collection::<ImageAssets>();
  }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
  #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 14, rows = 11, padding_x = 1, padding_y = 1))]
  pub layout: Handle<TextureAtlasLayout>,

  #[asset(path = "tilesheet.png")]
  pub ground: Handle<Image>
}
