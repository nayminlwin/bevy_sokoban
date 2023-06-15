use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Clone, Copy)]
pub struct MapSize {
    pub width: usize,
    pub height: usize
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct MoveTimer(pub Timer);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub player: Player,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub move_cooldown: MoveTimer,
    pub tile_pos: TilePos
}

#[derive(Debug, Component, Clone, Copy)]
pub struct TilePos {
    pub x: usize,
    pub y: usize,
}

impl TilePos {
    pub fn to_index(&self, max_width: usize) -> usize {
        self.x + self.y * max_width
    }

    pub fn add_and_clone(&self, dx: i32, dy: i32) -> TilePos {
        TilePos {
            x: (self.x as i32 + dx) as usize,
            y: (self.y as i32 + dy) as usize
        }
    }
}

#[derive(Component)]
pub struct Wall;
// #[derive(Component)]
// pub struct Trigger;
#[derive(Component)]
pub struct Box;
#[derive(Component)]
pub struct Door;

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


impl TileStorage {
    pub fn new(size: MapSize) -> Self {
        Self {
            tiles: vec![None; size.width * size.height],
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

pub fn assets_bundle(
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) -> (Handle<TextureAtlas>, Handle<TextureAtlas>) {

    let player_handle = asset_server.load("sprites/HumanBaseIdle.png");
    let player_atlas =
        TextureAtlas::from_grid(player_handle, 
        Vec2::new(8., 8.), 16, 4, Some(Vec2::splat(24.)), 
        Some(Vec2::splat(12.)));
    let player_atlas_handle = texture_atlases.add(player_atlas);

    let dungeon_handle = asset_server.load("sprites/DungeonTiles.png");
    let dungeon_atlas = 
        TextureAtlas::from_grid(dungeon_handle, 
        Vec2::new(8., 8.), 4, 1, None, None);
    let dungeon_atlas_handle = texture_atlases.add(dungeon_atlas);

    (player_atlas_handle, dungeon_atlas_handle)

    /* PlayerBundle {
        sprite_sheet_bundle: SpriteSheetBundle { 
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: player_atlas_handle, 
            ..default()
        },
        player: Player,
        animation_indices: AnimationIndices { first: 0, last: 15 },
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    } */
}
