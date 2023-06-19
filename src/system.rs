use bevy::prelude::*;

use crate::assets::*;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    ), With<Player>>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index >= indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn entity_update(time: Res<Time>,
    mut query: Query<(&mut Transform, &WorldPosition)>) {

    for (mut transform, movable) in &mut query {
        let delta = Vec3::new(
            (movable.x - transform.translation.x) * time.delta_seconds() * 5.,
            (movable.y - transform.translation.y) * time.delta_seconds() * 5., 0.);
        transform.translation += delta;
    }
}

pub fn player_move(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    triggers: Query<&TriggerIndices>,
    door_index_query: Query<&DoorIndex>,
    mut player: Query<(Entity, &mut WorldPosition, &mut TilePos, 
        &mut MoveTimer, &mut AnimationIndices, &mut TextureAtlasSprite), With<Player>>,
    mut blocking_tiles_query: Query<
        (Entity, &mut TilePos, &mut Transform, Option<&mut WorldPosition>, &mut TextureAtlasSprite, &BlockType),
        Without<Player>>,
    mut map_tiles_query: Query<&mut TileStorage>) {

    let mut map_tiles: Mut<TileStorage> = map_tiles_query.single_mut();
    for (player_entity, mut world_pos, mut tile_pos, mut move_cooldown, 
        mut anim_indices, mut sprite) in &mut player {

        if move_cooldown.tick(time.delta()).finished() {

            anim_indices.first = 0;
            anim_indices.last = 7;

            let mut movement = Vec3::ZERO;
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;
            if keyboard_input.just_pressed(KeyCode::Left) {
                dx -= 1;
                movement.x = -1. * 8.;
            } else if keyboard_input.just_pressed(KeyCode::Up) {
                dy -= 1;
                movement.y = 1. * 8.;
            } else if keyboard_input.just_pressed(KeyCode::Right) {
                dx += 1;
                movement.x = 1. * 8.;
            } else if keyboard_input.just_pressed(KeyCode::Down) {
                dy += 1;
                movement.y = -1. * 8.;
            } else {
                continue;
            }

            let new_pos = tile_pos.add_and_clone(dx, dy);

            // Flip sprite depending on x coord direction
            sprite.flip_x = dx < 0;

            // Block if new position is out of bounds
            if new_pos.x < 0 || new_pos.x >= map_tiles.size.width 
                || new_pos.y < 0 || new_pos.y >= map_tiles.size.height {
                return;
            }

            let old_index = tile_pos.to_index(map_tiles.size.width);
            let new_index = new_pos.to_index(map_tiles.size.width);

            if let Some(blocking_entity) = map_tiles.move_tile(
                player_entity, old_index, new_index) {

                let (box_entity, mut tile_pos, _, maybe_world_pos, _, block_type)
                    = blocking_tiles_query.get_mut(blocking_entity).unwrap();

                if matches!(block_type, BlockType::Box) {
                    let old_index = tile_pos.to_index(map_tiles.size.width);

                    let new_pos = tile_pos.add_and_clone(dx, dy);
                    let new_index = new_pos.to_index(map_tiles.size.width);

                    if let None = map_tiles.move_tile(box_entity, old_index, new_index) {
                        *tile_pos = new_pos;
                        let mut world_pos = maybe_world_pos.unwrap();
                        world_pos.x += movement.x;
                        world_pos.y += movement.y;
                    }

                    // Check if all triggers are activated
                    let mut win = true;
                    let TriggerIndices(triggers) = triggers.single();
                    for trigger_index in triggers {
                        if let Some(entity) = map_tiles.tiles[*trigger_index] {
                            match blocking_tiles_query.get(entity) {
                                Err(_) => {
                                    win = false;
                                    break;
                                },
                                Ok((_, _, _, _, _, block_type)) => {
                                    win = matches!(block_type, BlockType::Box);
                                }
                            }
                        } else {
                            win = false;
                            break;
                        }
                    }
                    if win {
                        let DoorIndex(door_index) = door_index_query.single();
                        if let Some(door_entity) = map_tiles.tiles[*door_index] {
                            let (_, _, mut transform, _, mut texture, block_type)
                                = blocking_tiles_query.get_mut(door_entity).unwrap();
                            if matches!(block_type, BlockType::Door) {
                                texture.index = 5; // Switch to opened door sprite
                                    transform.translation.z -= 1.;
                                map_tiles.tiles[*door_index] = None;
                            }
                        }
                    }
                }
            } else {
                *tile_pos = new_pos;
                sprite.index = 8;
                anim_indices.first = 8;
                anim_indices.last = 11;
                world_pos.x += movement.x;
                world_pos.y += movement.y;
            }

            move_cooldown.reset();
        }

    }
}
