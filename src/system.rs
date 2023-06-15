use bevy::prelude::*;

use crate::assets::*;

pub fn player_move(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    triggers: Query<&TriggerIndices>,
    mut player: Query<(Entity, &mut Transform, &mut TilePos, &mut MoveTimer), With<Player>>,
    mut tiles_query: Query<(Entity, &mut TilePos, &TileType, &mut Transform), Without<Player>>,
    mut map_tiles_query: Query<&mut TileStorage>) {

    let mut map_tiles: Mut<TileStorage> = map_tiles_query.single_mut();
    for (player_entity, mut transform, mut tile_pos, mut move_cooldown) in &mut player {
        if move_cooldown.tick(time.delta()).finished() {
            let mut movement = Vec3::ZERO;
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;
            if keyboard_input.pressed(KeyCode::Left) {
                dx -= 1;
                movement.x = -1. * 8.;
            } else if keyboard_input.pressed(KeyCode::Up) {
                dy -= 1;
                movement.y = 1. * 8.;
            } else if keyboard_input.pressed(KeyCode::Right) {
                dx += 1;
                movement.x = 1. * 8.;
            } else if keyboard_input.pressed(KeyCode::Down) {
                dy += 1;
                movement.y = -1. * 8.;
            } else {
                continue;
            }

            let new_pos = TilePos {
                x: (tile_pos.x as i32 + dx) as usize,
                y: (tile_pos.y as i32 + dy) as usize
            };

            let new_index = new_pos.to_index(map_tiles.size.width);

            if let Some(tile_entity) = map_tiles.tiles[new_index] {
                // If the new pos 
                let (box_entity, mut tile_pos, tile_type, mut transform) = tiles_query.get_mut(tile_entity).unwrap();
                if matches!(tile_type, TileType::BOX) {
                    let orig_index = tile_pos.to_index(map_tiles.size.width);
                    map_tiles.tiles[orig_index] = None;

                    tile_pos.x += (tile_pos.x as i32 + dx) as usize;
                    tile_pos.y += (tile_pos.y as i32 + dy) as usize;
                    let new_index = tile_pos.to_index(map_tiles.size.width);
                    map_tiles.tiles[new_index] = Some(box_entity);
                    transform.translation += movement;
                }
            } else {
                let orig_index = tile_pos.to_index(map_tiles.size.width);
                map_tiles.tiles[orig_index] = None;

                *tile_pos = new_pos;
                map_tiles.tiles[new_index] = Some(player_entity);
                transform.translation += movement;
            }
            move_cooldown.reset();
        }
    }
}
