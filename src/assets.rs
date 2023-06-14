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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub player: Player,

    pub animation_indices: AnimationIndices,

    pub animation_timer: AnimationTimer,

    pub tile_pos: TilePos
}

#[derive(Component, Clone, Copy)]
pub struct TilePos {
    pub x: usize,
    pub y: usize,
}

impl TilePos {
    pub fn to_index(&self, max_width: usize) -> usize {
        self.x + self.y * max_width
    }
}

#[derive(Component)]
pub enum TileType {
    WALL,
    TRIGGER,
    BOX,
    DOOR
}

#[derive(Bundle)]
pub struct TileBundle {
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,

    pub tile_type: TileType,
    pub tile_pos: TilePos
}

#[derive(Component)]
pub struct MapTiles{ 
    pub tiles: [Option<Entity>; 128 * 128],
    pub size: MapSize,
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
