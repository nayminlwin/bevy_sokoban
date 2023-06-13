use bevy::prelude::*;

use crate::{assets::*, levels::LEVELS};

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

    let player_handle = asset_server.load("sprites/HumanBaseIdle.png");
    let player_atlas =
        TextureAtlas::from_grid(player_handle, 
        Vec2::new(8., 8.), 16, 4, Some(Vec2::splat(24.)), 
        Some(Vec2::splat(12.)));
    let player_atlas_handle = texture_atlases.add(player_atlas);

    let dungeon_handle = asset_server.load("sprites/DungeonTiles.png");
    let dungeon_atlas = 
        TextureAtlas::from_grid(dungeon_handle, 
        Vec2::new(8., 8.), 4, 2, None, None);
    let atlas_handle = texture_atlases.add(dungeon_atlas);

    let map_level = LEVELS[0].trim();
    let mut map_size = MapSize { width: 0, height: 0 };

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

    x = 1;
    y = -1;

    // commands.spawn(map_size);

    let center_transform = Transform::from_xyz(
        -(map_size.width as f32 * 8.) / 2., 
        (map_size.height as f32 * 8.) / 2., 0.);
    for c in map_level.chars() {
        if c == '\n' {
            y -= 1;
            x = 1;
        } else {
            let transform = 
                Transform::from_xyz(x as f32 * 8., y as f32 * 8., 0.) 
                * center_transform;
            
            commands.spawn((
                Floor,
                create_tile_bundle(0, atlas_handle.clone(), transform),
            ));

            let transform = Transform::from_xyz(x as f32 * 8., y as f32 * 8., 1.) * center_transform;

            match c {
               '#' => {
                    commands.spawn((
                        Wall,
                        create_tile_bundle(2, atlas_handle.clone(), transform),
                    ));
                },
                '@' => {
                    commands.spawn(PlayerBundle {
                        sprite_sheet_bundle:
                            create_tile_bundle(0, player_atlas_handle.clone(), transform),
                        animation_timer: 
                            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                        animation_indices: AnimationIndices { first: 0, last: 15 },
                        player: Player,
                    });
                },
                'c' => {
                    commands.spawn((
                        Box,
                        create_tile_bundle(3, atlas_handle.clone(), transform),
                    ));
                },
                'o' => {
                    commands.spawn((
                        Trigger,
                        create_tile_bundle(1, atlas_handle.clone(), transform),
                    ));
                },
                'D' => {
                    commands.spawn((
                        Door,
                        create_tile_bundle(4, atlas_handle.clone(), transform),
                    ));
                },
                unknown => ()
            };
            x += 1;
        }
    }
}
