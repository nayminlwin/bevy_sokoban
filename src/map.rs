use bevy::prelude::*;

use crate::{assets::*, levels::LEVELS};
use crate::GameState;

fn create_tile_bundle(sprite_index: usize, texture_atlas: Handle<TextureAtlas>, transform: Transform)
    -> SpriteSheetBundle {

    SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(sprite_index), 
        texture_atlas, 
        transform,
        ..default()
    }
}

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    let player_atlas_handle = load_asset_atlas(&asset_server, &mut texture_atlases, 
        "sprites/HumanBaseIdle.png", 8, 2, Some(Vec2::splat(24.)), Some(Vec2::splat(12.)));

    let atlas_handle = load_asset_atlas(&asset_server, &mut texture_atlases,
        "sprites/DungeonTiles.png", 4, 2, None, None);

    let map_level = LEVELS[0].trim();
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
            
            commands.spawn(
                create_tile_bundle(0, atlas_handle.clone(), transform),
            );

            let tile_pos = TilePos::new(x, y, map_size.width);

            let transform = Transform::from_xyz(x as f32 * 8., y as f32 * -8., 3.) * center_transform;

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
                    move_cooldown: MoveTimer(Timer::from_seconds(0.4, TimerMode::Once))
                }).id();
                map_tiles.tiles[tile_pos.index] = Some(entity);
            } else if c == 'o' {
                let transform = Transform::from_xyz(x as f32 * 8., y as f32 * -8., 2.)
                    * center_transform;

                commands.spawn(
                    create_tile_bundle(1, atlas_handle.clone(), transform)
                );
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
    mut tiles_query: Query<&mut TilePos>) {
    commands.spawn(AnimationTimer(Timer::from_seconds(1., TimerMode::Once)));
}

pub fn clear_map(
    timer: Res<Timer>,
    mut tiles_q: Query<&mut Transform>,
    timer_q: Query<&AnimationTimer, Without<Player>>) {
    if let Ok(anim_timer) = timer_q.get_single() {
        if (anim_timer.tick(timer.delta()).finished()) {
            // TODO: Transition to level load
        } else {
            
        }
    }
}

pub fn clear_map(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>,
    entities: Query<Entity, (Without<Camera>, Without<Window>)>) {

    for entity in &entities {
        commands.entity(entity).despawn();
    }

    next_state.set(GameState::Playing);
}
