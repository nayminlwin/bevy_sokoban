use bevy::prelude::*;

use crate::assets::*;

pub fn player_move(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    triggers: Query<&TriggerIndices>,
    mut player: Query<(Entity, &mut Transform, &mut TilePos, &mut MoveTimer), With<Player>>,
    mut tiles_query: Query<(Entity, &mut TilePos, &mut Transform), (With<Box>, Without<Player>)>,
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

            let new_pos = tile_pos.add_and_clone(dx, dy);
            let old_index = tile_pos.to_index(map_tiles.size.width);
            let new_index = new_pos.to_index(map_tiles.size.width);

            if let Some(blocking_entity) = map_tiles.move_tile(
                player_entity, old_index, new_index) {

                let (box_entity, mut tile_pos, mut transform)
                    = tiles_query.get_mut(blocking_entity).unwrap();

                let old_index = tile_pos.to_index(map_tiles.size.width);

                let new_pos = tile_pos.add_and_clone(dx, dy);
                let new_index = new_pos.to_index(map_tiles.size.width);

                if let None = map_tiles.move_tile(box_entity, old_index, new_index) {
                    *tile_pos = new_pos;
                    transform.translation += movement;
                }

                // Check if all triggers are activated
                let mut win = true;
                let TriggerIndices(triggers) = triggers.single();
                for trigger_index in triggers {
                    if let Some(entity) = map_tiles.tiles[*trigger_index] {
                        if let Err(_) = tiles_query.get(entity) {
                            win = false;
                            break;
                        }
                    } else {
                        win = false;
                        break;
                    }
                }
                if win {
                    println!("Win!");
                }
            } else {
                *tile_pos = new_pos;
                transform.translation += movement;
            }

            move_cooldown.reset();
        }
    }
}
