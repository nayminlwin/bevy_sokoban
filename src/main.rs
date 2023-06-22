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

#[derive(Resource)]
pub struct GameLevel(usize);

// Manually implement default to control the start level
impl Default for GameLevel {
    fn default() -> Self {
        Self(0)
    }
}


pub fn load_asset_atlas(
    asset_server: &Res<AssetServer>,  
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

fn spawn_gameover(
    mut commands: Commands,
    asset_server: Res<AssetServer>,  
) {
    commands.spawn(
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Congratulations!\nYou've completed all the levels.\nPress 'Space' to start over.",
            TextStyle {
                font: asset_server.load("Minimal3x5.ttf"),
                font_size: 5.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                ..default()
            })
    );
}

fn clear_gameover(mut commands: Commands,
    text_q: Query<Entity, With<Text>>,
) {
    for entity in &text_q {
        commands.entity(entity).despawn();
    }
}

fn startover(
    mut game_level: ResMut<GameLevel>,
    mut next_state: ResMut<NextState<GameState>>, 
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_level.0 = 0;
        next_state.set(GameState::Starting);
    }
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
        .add_system(spawn_gameover.in_schedule(OnEnter(GameState::GameOver)))
        .add_system(clear_gameover.in_schedule(OnExit(GameState::GameOver)))
        .add_system(startover.in_set(OnUpdate(GameState::GameOver)))
        .run();
}
