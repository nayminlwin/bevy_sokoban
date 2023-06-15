use bevy::{prelude::*, window::WindowResolution};
pub mod assets;
pub mod levels;
pub mod map;
pub mod system;

use map::*;
use assets::*;
use system::*;

fn animate_sprite(
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
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { 
        transform: Transform {
            scale: Vec3::splat(0.2),
            translation: Vec3::new(0., 0., 5.),
            ..default()
        },
        ..default()
    });

}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Sprite Animation"),
                        resolution: WindowResolution::new(640., 400.),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_startup_systems((spawn_camera, spawn_map))
        .add_system(animate_sprite)
        .add_system(player_move)
        .run();
}
