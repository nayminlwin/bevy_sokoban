use bevy::{prelude::*, window::WindowResolution};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    NextLevel,
    GameOver
}

#[derive(Resource, Default)]
pub struct GameLevel(usize);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { 
        transform: Transform {
            translation: Vec3::new(0., 0., 10.),
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
                        title: String::from("Sokoban"),
                        resolution: WindowResolution::new(640., 400.).with_scale_factor_override(5.),
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
        .init_resource::<GameLevel>()
        .add_startup_system(spawn_camera)
        .add_system(spawn_map.in_schedule(OnEnter(GameState::Playing)))
        .add_systems((
                animate_sprite,
                player_move,
                entity_update,
                win_condition,
                reset_map, 
             ).in_set(OnUpdate(GameState::Playing)))
        .add_system(init_clear_map.in_schedule(OnEnter(GameState::Resetting)))
        .add_system(init_clear_map.in_schedule(OnEnter(GameState::NextLevel)))
        .add_system(clear_map.in_set(OnUpdate(GameState::Resetting)))
        .add_system(clear_map.in_set(OnUpdate(GameState::NextLevel)))
        .run();
}
