use bevy::{prelude::*, window::WindowResolution, render::camera::Viewport};

pub mod assets;
pub mod levels;
pub mod map;

use map::*;
use assets::*;

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
        camera: Camera {
            viewport: Some(Viewport {
                physical_size: UVec2::new(640, 400),
                ..default()
            }),
            ..default()
        },
        transform: Transform::from_scale(Vec3::splat(0.3)),
        ..default()
    });

}

fn main() {
    dbg!(levels::LEVELS);
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
        .run();
}
