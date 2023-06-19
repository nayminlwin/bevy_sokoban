use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
pub mod assets;
pub mod levels;
pub mod map;
pub mod system;

use map::*;
use system::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Playing,
    Resetting,
    GameOver
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { 
        transform: Transform {
            scale: Vec3::splat(0.2),
            translation: Vec3::new(0., 0., 5.),
            ..default()
        },
        ..default()
    });

}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Sprite Animation"),
                        resolution: WindowResolution::new(640., 400.),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
        )
        .add_state::<GameState>()
        .add_startup_system(spawn_camera)
        .add_system(spawn_map.in_schedule(OnEnter(GameState::Playing)))
        .add_systems((animate_sprite, player_move, reset_map, entity_update)
            .in_set(OnUpdate(GameState::Playing)))
        .add_system(clear_map.in_schedule(OnEnter(GameState::Resetting)))
        .run();
}
