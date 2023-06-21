use bevy::{prelude::*, window::WindowResolution};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
pub mod components;
pub mod levels;
pub mod map;
pub mod player;

use map::*;
use player::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Starting,
    Playing,
    Resetting,
    NextLevel,
    GameOver
}

#[derive(Resource, Default)]
pub struct GameLevel(usize);


pub fn load_asset_atlas(asset_server: &Res<AssetServer>,  
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    path: &str, columns: usize, rows: usize,
    padding: Option<Vec2>, offset: Option<Vec2>) -> Handle<TextureAtlas> {

    let asset_handle = asset_server.load(path);
    let asset_atlas =
        TextureAtlas::from_grid(asset_handle, 
        Vec2::new(8., 8.), columns, rows, padding, offset);
    return texture_atlases.add(asset_atlas);
}


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
        .add_system(spawn_map.in_schedule(OnEnter(GameState::Starting)))
        .add_system(transition_map.in_set(OnUpdate(GameState::Starting)))
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
