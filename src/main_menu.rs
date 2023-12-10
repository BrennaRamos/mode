use crate::{game_mod, AppState};
use bevy::render::color::*;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

const OLIVE_GREEN: Color = Color::rgb(82.0 / 255.0, 88.0 / 255.0, 32.0 / 255.0);

#[derive(Component)]
pub enum ActionButton {
    Play,
    Leaderboard,
    HowToPlay,
    Quit,
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera2d>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for entity in camera_query.iter() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    // Spawn Camera in Foreground
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BISQUE),
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..default()
    });
    // Spawn Menu Title
    let texture_handle = asset_server.load("Icons/Title.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(960.0, 540.0), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 5 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_translation(Vec3::new(50.0, 150.0, 0.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));
    // Spawn Menu Buttons
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // bottom left button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            ActionButton::Play,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(65.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    ActionButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: OLIVE_GREEN,
                            },
                        ),
                        ActionButton::Play,
                    ));
                });
        })
        // Spawn Leaderboard Button
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.),
                            height: Val::Px(65.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    ActionButton::Leaderboard,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Leaderboard",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: OLIVE_GREEN,
                            },
                        ),
                        ActionButton::Leaderboard,
                    ));
                });
        })
        // Spawn How To Play Button
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.),
                            height: Val::Px(65.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    ActionButton::HowToPlay,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "How to Play",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: OLIVE_GREEN,
                            },
                        ),
                        ActionButton::HowToPlay,
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(65.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    ActionButton::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Quit",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: OLIVE_GREEN,
                            },
                        ),
                        ActionButton::Quit,
                    ));
                });
        });
}

pub fn animate_menu_title(
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

pub fn interact_menu(
    mut interaction_query: Query<
        (&Interaction, &ActionButton, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        next_state.set(AppState::QuitGame);
    }

    if keyboard_input.just_released(KeyCode::Return) {
        next_state.set(AppState::StartRound);
    }
    // Buttons
    for (interaction, answer_button, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = Color::WHITE.into();
                match answer_button {
                    ActionButton::HowToPlay => next_state.set(AppState::HowToPlay),
                    ActionButton::Play => next_state.set(AppState::StartRound),
                    ActionButton::Leaderboard => next_state.set(AppState::Leaderboard),
                    ActionButton::Quit => next_state.set(AppState::QuitGame),
                }
            }
            Interaction::Hovered => {
                *border_color = Color::SALMON.into();
            }
            Interaction::None => {
                *border_color = OLIVE_GREEN.into();
            }
        }
    }
}

pub fn clear_shapes(
    mut commands: Commands,
    mut query: Query<Entity, With<ActionButton>>,
    mut query_title: Query<Entity, With<AnimationTimer>>,
    current_state: Res<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    for entity in query.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    for entity in query_title.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }

    match current_state.get() {
        AppState::StartRound => {
            game_mod::setup_ui(&mut commands, &asset_server);
        }
        _ => {}
    }
}

pub fn quit_game(mut commands: Commands, mut query: Query<Entity>) {
    for entity in query.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
