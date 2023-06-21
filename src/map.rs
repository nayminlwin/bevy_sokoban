use bevy::prelude::*;

use crate::{assets::*, levels::LEVELS};
use crate::{GameState, GameLevel};

fn create_tile_bundle(sprite_index: usize, texture_atlas: Handle<TextureAtlas>, transform: Transform)
    -> SpriteSheetBundle {

    SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(sprite_index), 
        texture_atlas, 
        transform,
        ..default()
    }
}

pub fn spawn_map(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    game_level: Res<GameLevel>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    let player_atlas_handle = load_asset_atlas(&asset_server, &mut texture_atlases, 
        "sprites/HumanBaseIdle.png", 8, 2, Some(Vec2::splat(24.)), Some(Vec2::splat(12.)));

    let atlas_handle = load_asset_atlas(&asset_server, &mut texture_atlases,
        "sprites/DungeonTiles.png", 4, 2, None, None);

    let level = game_level.0;
    let map_level = LEVELS[level].trim();
    let mut map_size = MapSize { width: 0, height: 1 };
    let mut triggers = Vec::new();

    let mut x = 0;
    let mut y = 0;

    for c in map_level.chars() {
        if c == '\n' {
            if map_size.width < x {
                map_size.width = x;
            }
            x = 0;
            map_size.height += 1;
        } else {
            x += 1;
        }
    }

    let mut map_tiles = TileStorage::new(map_size);

    x = 0;

    let center_transform = Transform::from_xyz(
        -(map_size.width as f32 * 8.) / 2., 
        (map_size.height as f32 * 8.) / 2., 0.);

    for c in map_level.chars() {
        if c == '\n' {
            y += 1;
            x = 0;
        } else {
            let transform = 
                Transform::from_xyz(x as f32 * 8., y as f32 * -8., 0.) 
                    * center_transform;
            
            let tile_pos = TilePos::new(x, y, map_size.width);

            commands.spawn(
                (create_tile_bundle(0, atlas_handle.clone(), transform),
                    tile_pos,
                ));

            let mut transform = Transform::from_xyz(x as f32 * 8., y as f32 * -8., 3.) * center_transform;
            if c == 'D' {
                // Rotate the door if on the sides
                if x == 0 {
                    transform.rotate_z(1.570796);
                } else if x + 1 == map_size.width {
                    transform.rotate_z(-1.570796);
                }
            }

            if c == '@' {
                let entity = commands.spawn(PlayerBundle {
                    sprite_sheet_bundle:
                        create_tile_bundle(0, player_atlas_handle.clone(), transform),
                    player: Player,
                    movable: WorldPosition {
                        x: transform.translation.x,
                        y: transform.translation.y
                    },
                    tile_pos,
                    animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    animation_indices: AnimationIndices { first: 0, last: 7 },
                    move_cooldown: MoveTimer(Timer::from_seconds(0.3, TimerMode::Once))
                }).id();
                map_tiles.tiles[tile_pos.index] = Some(entity);
            } else if c == 'o' {
                let transform = Transform::from_xyz(x as f32 * 8., y as f32 * -8., 2.)
                    * center_transform;

                commands.spawn((
                    create_tile_bundle(1, atlas_handle.clone(), transform),
                    tile_pos
                ));

                triggers.push(tile_pos.index);
            } else if c == 'b' {
                let entity = commands.spawn((
                    create_tile_bundle(3, atlas_handle.clone(), transform),
                    tile_pos, BlockType::Box,
                    WorldPosition {
                        x: transform.translation.x,
                        y: transform.translation.y
                    },
                )).id();
                map_tiles.tiles[tile_pos.index] = Some(entity);
            }

            if let Some((sprite_index, block_type)) = match c {
                '#' => Some((2, BlockType::Wall)),
                'D' => {
                    commands.spawn(DoorIndex(tile_pos.index));
                    Some((4, BlockType::Door))
                },
                _ => None
            } {
                let entity = commands.spawn((
                    create_tile_bundle(sprite_index, atlas_handle.clone(), transform),
                    tile_pos, block_type
                )).id();
                map_tiles.tiles[tile_pos.index] = Some(entity);
            }

            x += 1;
        }

    }
    commands.spawn(map_tiles);
    commands.spawn(TriggerIndices(triggers));
}

pub fn reset_map(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::R) {
        next_state.set(GameState::Resetting);
    }
}

pub fn init_clear_map(
    mut commands: Commands,
    // map_tiles_query: Query<&TileStorage>,
) {
    let world_pos = WorldPosition {
            x: 180.,
            y: 120.,
        };
    commands.spawn(( 
        AnimationTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        world_pos
    ));
}

pub fn clear_map(
    mut commands: Commands,
    timer: Res<Time>,
    game_state: Res<State<GameState>>,
    mut game_level: ResMut<GameLevel>,
    mut game_state_next: ResMut<NextState<GameState>>,
    mut tiles_q: Query<(Entity, Option<&TilePos>, Option<&mut Transform>), (Without<Camera>, Without<Window>)>,
    mut timer_q: Query<(&mut AnimationTimer, &WorldPosition), Without<Player>>) {
    if let Ok((mut anim_timer, world_pos)) = timer_q.get_single_mut() {
        let time_delta = timer.delta();
        if anim_timer.tick(time_delta).finished() {
            // TODO: clear entities and transition to level load

            for (entity, _, _) in &tiles_q {
                commands.entity(entity).despawn();
            }
            if matches!(game_state.0, GameState::NextLevel) {
                let level = game_level.0;
                let new_level = level + 1;
                if new_level < LEVELS.len() {
                    game_level.0 = new_level;
                    game_state_next.set(GameState::Playing);
                } else {
                    game_state_next.set(GameState::GameOver);
                }
            } else {
                // Assume to be just resetting the level
                game_state_next.set(GameState::Playing);
            }
        } else {
            let elapsed = anim_timer.percent();
            if elapsed <= 1. {
                for (_, maybe_tilepos, maybe_transform) in &mut tiles_q {
                    if let Some(mut transform) = maybe_transform {
                        if let Some(tile_pos) = maybe_tilepos {
                            let offset: f32 = (((tile_pos.x as f32 - 2.) / 50.) + elapsed).clamp(0., 1.);
                            transform.translation.y += 
                                (world_pos.y - transform.translation.y) * offset;
                        }
                    }
                }
            }
        }
    }
}

