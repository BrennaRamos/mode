use crate::AppState;
use bevy::render::camera::ScalingMode;
use bevy::render::color::*;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

#[derive(Component)]
pub struct Loading;

#[derive(Resource)]
pub struct LoadingTimer {
    pub loading_timer: Timer,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub const OLIVE_GREEN: Color = Color::rgb(82.0 / 255.0, 88.0 / 255.0, 32.0 / 255.0);

pub fn setup_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn Camera in Foreground
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BISQUE),
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 1280.0,
                height: 720.0,
            },
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..default()
    });

    // Spawn Background
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("background/background.png"),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
    //     ..default()
    // });

    // Spawn Animated Background
    let texture_handle_sun = asset_server.load("background/background.png");
    let texture_atlas_sun = TextureAtlas::from_grid(
        texture_handle_sun,
        Vec2::new(1280.0, 720.0),
        5,
        1,
        None,
        None,
    );
    let texture_atlas_handle_sun = texture_atlases.add(texture_atlas_sun);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices_sun = AnimationIndices { first: 0, last: 4 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_sun,
            sprite: TextureAtlasSprite::new(animation_indices_sun.first),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        animation_indices_sun,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));

    // Spawn Loading Sprite
    let texture_handle = asset_server.load("icons/loading.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(80.0, 80.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 6 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        animation_indices,
        Loading,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));

    // Spawn Loading Text
    let font = asset_server.load("fonts/Leila-Regular.ttf");
    commands.spawn((
        {
            TextBundle {
                text: Text::from_section(
                    "Loading",
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: OLIVE_GREEN,
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    top: Val::Px(50.0),
                    ..default()
                },
                ..default()
            }
        },
        Loading,
    ));
}

pub fn tick_loading(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    timer.loading_timer.tick(time.delta());

    if timer.loading_timer.finished() {
        next_state.set(AppState::MainMenu);
    }
}

pub fn animate_background_and_load(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
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

pub fn clear_shapes(mut commands: Commands, mut query: Query<Entity, With<Loading>>) {
    for entity in query.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
