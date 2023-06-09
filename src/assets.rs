use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Floor;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub player: Player,

    pub animation_indices: AnimationIndices,

    pub animation_timer: AnimationTimer,
}

#[derive(Bundle)]
pub struct FloorBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub floor: Floor, 
}


pub fn assets_bundle(
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) -> (PlayerBundle, FloorBundle) {

    let player_handle = asset_server.load("sprites/HumanBaseIdle.png");
    let player_atlas =
        TextureAtlas::from_grid(player_handle, 
        Vec2::new(8., 8.), 16, 4, Some(Vec2::splat(24.)), 
        Some(Vec2::splat(12.)));
    let player_atlas_handle = texture_atlases.add(player_atlas);

    let floor_handle = asset_server.load("sprites/Minifantasy_DungeonFloorTiles.png");
    let floor_atlas = 
        TextureAtlas::from_grid(floor_handle, 
        Vec2::new(8., 8.), 7, 2, None, None);
    let floor_atlas_handle = texture_atlases.add(floor_atlas);

    (PlayerBundle {
        sprite_sheet_bundle: SpriteSheetBundle { 
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: player_atlas_handle, 
            ..default()
        },
        player: Player,
        animation_indices: AnimationIndices { first: 0, last: 15 },
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    },
    FloorBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: floor_atlas_handle,
            ..default()
        },
        floor: Floor
    })
}
