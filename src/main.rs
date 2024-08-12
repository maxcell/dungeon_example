use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use dungeon_example::{assets::ImageAssets, controls::PlayerAction, map::{spawn_camera, spawn_world, update_camera}, player::{move_player, spawn_player}};
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TilemapPlugin))
        .init_collection::<ImageAssets>()
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .add_systems(Startup, (spawn_camera, spawn_world, spawn_light, spawn_player))
        .add_systems(FixedUpdate, (move_player, update_camera).chain())
    .run();
}



fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
