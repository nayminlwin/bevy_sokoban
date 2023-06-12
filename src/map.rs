use bevy::prelude::*;

use crate::{assets::*, levels::LEVELS};

fn create_tile_bundle(sprite_index: usize, atlas_handle: Handle<TextureAtlas>, position: Vec3)
    -> SpriteSheetBundle {

    SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(sprite_index), 
        texture_atlas: atlas_handle, 
        transform: Transform::from_translation(position), 
        ..default()
    }
}

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {

    let dungeon_handle = asset_server.load("sprites/DungeonTiles.png");
    let dungeon_atlas = 
        TextureAtlas::from_grid(dungeon_handle, 
        Vec2::new(8., 8.), 4, 1, None, None);
    let atlas_handle = texture_atlases.add(dungeon_atlas);

    let mut x = 1;
    let mut y = -1;
    for c in LEVELS[0].trim().chars() {
        if c == '\n' {
            y -= 1;
            x = 1;
        } else {
            let pos = Vec3::new(x as f32 * 8., y as f32 * 8., 0.) + Vec3::new(-50.,50., 0.);
            match c {
               '#' => { 
                    println!("Printing wall at pos {}", pos);
                    commands.spawn((
                        Wall,
                        create_tile_bundle(2, atlas_handle.clone(), pos),
                    ));
                },
                '.'|'@' => {
                    commands.spawn((
                        Floor,
                        create_tile_bundle(0, atlas_handle.clone(), pos),
                    ));
                },

                'c' => { 
                    commands.spawn((
                        Box,
                        create_tile_bundle(3, atlas_handle.clone(), pos),
                    ));
                },
                'o' => { commands.spawn((
                        Trigger,
                        create_tile_bundle(1, atlas_handle.clone(), pos),
                    ));
                },
                unknown => println!("Unknown character {unknown} in level file")
            };
            x += 1;
        }
    }
}
