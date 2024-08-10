use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use dungeon_example::{assets::ImageAssets, map::{spawn_camera, spawn_world}, player::spawn_player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_collection::<ImageAssets>()
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .add_systems(Startup, (spawn_camera, spawn_world, spawn_light, spawn_player))
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
