use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct MapSize {
    pub width: i32,
    pub height: i32
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct MoveTimer(pub Timer);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Copy, Clone, Debug, Default, Component)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32
}

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub player: Player,
    pub world_pos: WorldPosition,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub move_cooldown: MoveTimer,
    pub tile_pos: TilePos
}

#[derive(Debug, Component, Clone, Copy)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
    pub index: usize,
}

impl TilePos {
    pub fn new(x: i32, y: i32, width: i32) -> Self {
        Self {
            x,
            y,
            index: (x + y * width) as usize
        }
    }

    pub fn add_and_clone(&self, dx: i32, dy: i32, width: i32) -> TilePos {
        let x = self.x + dx;
        let y = self.y + dy;
        TilePos {
            x,
            y,
            index: (x + y * width) as usize
        }
    }
}

#[derive(Component)]
pub enum BlockType { Wall, Box, Door }

/* #[derive(Bundle)]
pub struct TileBundle {
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
    pub tile_type: TileType,
    pub tile_pos: TilePos
}
 */

#[derive(Component)]
pub struct TileStorage {
    pub tiles: Vec<Option<Entity>>,
    pub size: MapSize,
}

#[derive(Component)]
pub struct TriggerIndices(pub Vec<usize>);

#[derive(Component)]
pub struct DoorIndex(pub usize);


impl TileStorage {
    pub fn new(size: MapSize) -> Self {
        Self {
            tiles: vec![None; (size.width * size.height) as usize],
            size
        }
    }

    pub fn move_tile(&mut self, 
                     entity: Entity, 
                     old_index: usize, 
                     new_index: usize) -> Option<Entity> {

        if let Some(blocking) = self.tiles[new_index] {
            return Some(blocking);
        } else {
            self.tiles[old_index] = None;
            self.tiles[new_index] = Some(entity);
            return None;
        }
    }
}

