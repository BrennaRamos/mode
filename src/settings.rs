use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_tweening::{
    lens::{TransformRotationLens, UiPositionLens},
    Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};

use crate::{
    game_mod::{FruitType, GridIdentifier},
    main_menu::{SoundEffect, BASIL_GREEN, FONT, OLIVE_GREEN, SKY_BLUE},
    AppState,
};

#[derive(Component)]
pub enum BackButton {
    MainMenu,
}

#[derive(Component)]
pub struct Hovered {}

#[derive(Component)]
pub struct Pressed {}

#[derive(Component)]
pub struct CharacterButton {}

#[derive(Resource)]
pub struct GameSettings {
    pub fruit_a: FruitType,
    pub fruit_b: FruitType,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fruit_a: FruitType::Apple,
            fruit_b: FruitType::Pear,
        }
    }
}

#[derive(Resource)]
pub struct Villagers {
    pub villagers: Vec<(String, String, bool, i32)>,
}

impl Default for Villagers {
    fn default() -> Self {
        Self {
            villagers: vec![
                (
                    "characters/baker.png".to_string(),
                    "characters/baker_solo.png".to_string(),
                    false,
                    6,
                ),
                (
                    "characters/bug_collector.png".to_string(),
                    "characters/bug_collector_solo.png".to_string(),
                    false,
                    11,
                ),
                (
                    "characters/traveler.png".to_string(),
                    "characters/traveler_solo.png".to_string(),
                    false,
                    21,
                ),
                (
                    "characters/farmer.png".to_string(),
                    "characters/farmer_solo.png".to_string(),
                    false,
                    31,
                ),
                (
                    "characters/gardener.png".to_string(),
                    "characters/gardener_solo.png".to_string(),
                    false,
                    41,
                ),
                (
                    "characters/librarian.png".to_string(),
                    "characters/librarian_solo.png".to_string(),
                    false,
                    51,
                ),
                (
                    "characters/merchant.png".to_string(),
                    "characters/merchant_solo.png".to_string(),
                    false,
                    61,
                ),
                (
                    "characters/penguin.png".to_string(),
                    "characters/penguin_solo.png".to_string(),
                    false,
                    81,
                ),
                (
                    "characters/student.png".to_string(),
                    "characters/student_solo.png".to_string(),
                    false,
                    91,
                ),
                (
                    "characters/cat.png".to_string(),
                    "characters/cat_solo.png".to_string(),
                    false,
                    101,
                ),
            ],
        }
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Music
    commands.spawn(AudioBundle {
        source: asset_server.load("music/Rain.ogg"),
        settings: PlaybackSettings {
            volume: Volume::new_relative(0.2),
            ..Default::default()
        },
        ..default()
    });
    // Spawn Title Text
    let title = format!("Village");

    commands.spawn({
        TextBundle {
            text: Text::from_section(
                title,
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 64.0,
                    color: OLIVE_GREEN,
                },
            )
            .with_alignment(TextAlignment::Center),
            style: Style {
                top: Val::Vh(-30.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        }
    });
    // Spawn Subtitle Text
    let title = format!(
        "Select the type of fruit you see in game.
Light green is Fruit A, light blue is Fruit B."
    );

    commands.spawn({
        TextBundle {
            text: Text::from_section(
                title,
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: 32.0,
                    color: OLIVE_GREEN,
                },
            )
            .with_alignment(TextAlignment::Center),
            style: Style {
                top: Val::Vh(-20.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        }
    });

    // Spawn Menu Buttons
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // bottom left button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    ..default()
                },
                ..default()
            },
            BackButton::MainMenu,
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
                                top: Val::Px(4.),
                                left: Val::Px(4.),
                                bottom: Val::Px(4.),
                                right: Val::Px(4.),
                            },
                            ..default()
                        },
                        background_color: Color::BISQUE.into(),
                        ..default()
                    },
                    BackButton::MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Back",
                            TextStyle {
                                font: asset_server.load("fonts/Leila-Regular.ttf"),
                                font_size: 40.0,
                                color: OLIVE_GREEN,
                            },
                        ),
                        BackButton::MainMenu,
                    ));
                });
        });
}

pub fn interact_button(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &BackButton, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("music/Back.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..Default::default()
                },
                ..default()
            },
            SoundEffect,
        ));
        next_state.set(AppState::MainMenu);
    }
    // Buttons
    for (interaction, answer_button, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("music/Select.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            ..Default::default()
                        },
                        ..default()
                    },
                    SoundEffect,
                ));
                *border_color = Color::WHITE.into();
                match answer_button {
                    BackButton::MainMenu => next_state.set(AppState::MainMenu),
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

pub fn hover_fruit(
    mut commands: Commands,
    mut interaction_query: Query<
        (Ref<Interaction>, &mut BorderColor, &FruitType, Entity),
        With<Button>,
    >,
    game_settings: ResMut<GameSettings>,
) {
    // Buttons
    for (interaction, mut border_color, fruit_type, entity) in interaction_query.iter_mut() {
        if interaction.is_changed() {
            match *interaction {
                Interaction::Pressed => {
                    *border_color = Color::WHITE.into();
                    commands.entity(entity).insert(Pressed {});
                }
                Interaction::Hovered => {
                    *border_color = Color::SALMON.into();
                    commands.entity(entity).insert(Hovered {});
                }
                Interaction::None => {
                    if *fruit_type == game_settings.fruit_a {
                        *border_color = BASIL_GREEN.into();
                    } else if *fruit_type == game_settings.fruit_b {
                        *border_color = SKY_BLUE.into();
                    } else {
                        *border_color = OLIVE_GREEN.into();
                    }
                    commands.entity(entity).remove::<Hovered>();
                    commands.entity(entity).remove::<Pressed>();
                }
            }
        } else if *interaction == Interaction::None {
            if *fruit_type == game_settings.fruit_a {
                *border_color = BASIL_GREEN.into();
            } else if *fruit_type == game_settings.fruit_b {
                *border_color = SKY_BLUE.into();
            } else {
                *border_color = OLIVE_GREEN.into();
            }
            commands.entity(entity).remove::<Hovered>();
        }
    }
}

pub fn set_fruits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<&FruitType, (With<Button>, With<Hovered>)>,
    mouse_input: Res<Input<MouseButton>>,
    mut game_settings: ResMut<GameSettings>,
) {
    if mouse_input.just_released(MouseButton::Left) {
        for fruit_type in interaction_query.iter_mut() {
            if fruit_type != &game_settings.fruit_b {
                game_settings.fruit_a = fruit_type.clone();
            }
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("music/Select.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..Default::default()
                    },
                    ..default()
                },
                SoundEffect,
            ));
        }
    }
    if mouse_input.just_released(MouseButton::Right) {
        for fruit_type in interaction_query.iter_mut() {
            if fruit_type != &game_settings.fruit_a {
                game_settings.fruit_b = fruit_type.clone();
            }
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("music/Select.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..Default::default()
                    },
                    ..default()
                },
                SoundEffect,
            ));
        }
    }
}

pub fn spawn_chibi(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    villagers: Res<Villagers>,
) {
    let columns = 10;
    let rows = 1;

    // Spawn Villager Title
    let title = format!("Villagers Unlocked");

    commands
        .spawn(NodeBundle {
            style: Style {
                // horizontally center child text
                justify_self: JustifySelf::Center,
                // vertically center child text
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn({
                TextBundle {
                    text: Text::from_section(
                        title,
                        TextStyle {
                            font: asset_server.load(FONT),
                            font_size: 32.0,
                            color: OLIVE_GREEN,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    style: Style {
                        top: Val::Vh(10.0),
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                }
            });

            // Spawn Villager Grid
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // Use the CSS Grid algorithm for laying out this node
                            display: Display::Grid,
                            top: Val::Vh(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    GridIdentifier::Grid,
                ))
                .with_children(|builder| {
                    builder
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    height: Val::Px(125.0),
                                    width: Val::Vw(80.0),
                                    // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                                    // As the height is set explicitly, this means the width will adjust to match the height
                                    aspect_ratio: Some(1.0),
                                    // Use grid layout for this node
                                    display: Display::Grid,
                                    // Set the grid to have 10 columns all with sizes minmax(0, 1fr)
                                    // This creates 10 exactly evenly sized columns
                                    grid_template_columns: RepeatedGridTrack::flex(columns, 1.0),
                                    // Set the grid to have 1 rows all with sizes minmax(0, 1fr)
                                    // This creates 1 exactly evenly sized rows
                                    grid_template_rows: RepeatedGridTrack::flex(rows, 1.0),
                                    justify_self: JustifySelf::Center,
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            GridIdentifier::Grid,
                        ))
                        .with_children(|builder| {
                            for (index, villager) in villagers.villagers.iter().enumerate() {
                                item_rect(
                                    builder,
                                    &asset_server,
                                    villager.0.clone(),
                                    villager.2.clone(),
                                    index % 2 == 0,
                                );
                            }
                        });
                });
        });
}

pub fn spawn_fruit(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                // horizontally center child text
                justify_self: JustifySelf::Center,
                // vertically center child text
                align_self: AlignSelf::Center,
                top: Val::Vh(-10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                    FruitType::Apple,
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
                                    // center the node vertically and horizontally within the window
                                    position_type: PositionType::Relative,
                                    border: UiRect {
                                        top: Val::Px(4.),
                                        left: Val::Px(4.),
                                        bottom: Val::Px(4.),
                                        right: Val::Px(4.),
                                    },
                                    margin: UiRect {
                                        left: Val::Px(30.0),
                                        right: Val::Px(30.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::BISQUE.into(),
                                ..default()
                            },
                            FruitType::Apple,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server.load("icons/apple.png"),
                                        ..default()
                                    },
                                    ..default()
                                },
                                FruitType::Apple,
                                Animator::new(
                                    Tween::new(
                                        EaseFunction::CubicInOut,
                                        Duration::from_millis(500),
                                        TransformRotationLens {
                                            start: Quat::from_rotation_z(10_f32.to_radians()),
                                            end: Quat::from_rotation_z(350_f32.to_radians()),
                                        },
                                    )
                                    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                                    .with_repeat_count(RepeatCount::Infinite),
                                ),
                            ));
                        });
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // bottom right button
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                    FruitType::Pear,
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
                                    // center the node vertically and horizontally within the window
                                    position_type: PositionType::Relative,
                                    border: UiRect {
                                        top: Val::Px(4.),
                                        left: Val::Px(4.),
                                        bottom: Val::Px(4.),
                                        right: Val::Px(4.),
                                    },
                                    margin: UiRect {
                                        left: Val::Px(30.0),
                                        right: Val::Px(30.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::BISQUE.into(),
                                ..default()
                            },
                            FruitType::Pear,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server.load("icons/pear.png"),
                                        ..default()
                                    },
                                    ..default()
                                },
                                FruitType::Pear,
                                Animator::new(
                                    Tween::new(
                                        EaseFunction::CubicInOut,
                                        Duration::from_millis(500),
                                        TransformRotationLens {
                                            start: Quat::from_rotation_z(10_f32.to_radians()),
                                            end: Quat::from_rotation_z(350_f32.to_radians()),
                                        },
                                    )
                                    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                                    .with_repeat_count(RepeatCount::Infinite),
                                ),
                            ));
                        });
                });

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // bottom left button
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                    FruitType::Orange,
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
                                    // center the node vertically and horizontally within the window
                                    position_type: PositionType::Relative,
                                    border: UiRect {
                                        top: Val::Px(4.),
                                        left: Val::Px(4.),
                                        bottom: Val::Px(4.),
                                        right: Val::Px(4.),
                                    },
                                    margin: UiRect {
                                        left: Val::Px(30.0),
                                        right: Val::Px(30.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::BISQUE.into(),
                                ..default()
                            },
                            FruitType::Orange,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server.load("icons/orange.png"),
                                        ..default()
                                    },
                                    ..default()
                                },
                                FruitType::Orange,
                                Animator::new(
                                    Tween::new(
                                        EaseFunction::CubicInOut,
                                        Duration::from_millis(500),
                                        TransformRotationLens {
                                            start: Quat::from_rotation_z(10_f32.to_radians()),
                                            end: Quat::from_rotation_z(350_f32.to_radians()),
                                        },
                                    )
                                    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                                    .with_repeat_count(RepeatCount::Infinite),
                                ),
                            ));
                        });
                });
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            // bottom left button
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                    FruitType::Strawberry,
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
                                    // center the node vertically and horizontally within the window
                                    position_type: PositionType::Relative,
                                    margin: UiRect {
                                        left: Val::Px(30.0),
                                        right: Val::Px(30.0),
                                        ..default()
                                    },
                                    border: UiRect {
                                        top: Val::Px(4.),
                                        left: Val::Px(4.),
                                        bottom: Val::Px(4.),
                                        right: Val::Px(4.),
                                    },

                                    ..default()
                                },
                                background_color: Color::BISQUE.into(),
                                ..default()
                            },
                            FruitType::Strawberry,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        texture: asset_server.load("icons/strawberry.png"),
                                        ..default()
                                    },
                                    ..default()
                                },
                                FruitType::Strawberry,
                                Animator::new(
                                    Tween::new(
                                        EaseFunction::CubicInOut,
                                        Duration::from_millis(500),
                                        TransformRotationLens {
                                            start: Quat::from_rotation_z(10_f32.to_radians()),
                                            end: Quat::from_rotation_z(350_f32.to_radians()),
                                        },
                                    )
                                    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                                    .with_repeat_count(RepeatCount::Infinite),
                                ),
                            ));
                        });
                });
        });
}

pub fn clear_shapes(
    mut commands: Commands,
    mut query: Query<Entity, With<BackButton>>,
    mut query_title: Query<Entity, With<Text>>,
    mut query_stamps: Query<Entity, With<Sprite>>,
    mut query_fruits: Query<Entity, With<Node>>,
    mut query_music: Query<Entity, (With<PlaybackSettings>, Without<SoundEffect>)>,
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
    for entity in query_stamps.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    for entity in query_fruits.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    for entity in query_music.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}

fn item_rect(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    char_file: String,
    unlocked: bool,
    bounce_type: bool,
) {
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            },
            GridIdentifier::Grid,
        ))
        .with_children(|builder| {
            builder.spawn((
                ImageBundle {
                    style: Style {
                        height: Val::Px(125.0),
                        width: Val::Px(100.0),
                        ..default()
                    },
                    image: asset_server.load(char_file).into(),
                    background_color: (if unlocked {
                        Color::WHITE.into()
                    } else {
                        Color::GRAY.into()
                    }),
                    ..default()
                },
                GridIdentifier::Grid,
                Animator::new(
                    Tween::new(
                        if bounce_type {
                            EaseFunction::BounceIn
                        } else {
                            EaseFunction::BounceOut
                        },
                        Duration::from_secs(1),
                        UiPositionLens {
                            start: UiRect::top(Val::Px(10.0)),
                            end: UiRect::top(Val::Px(0.0)),
                        },
                    )
                    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                    .with_repeat_count(RepeatCount::Infinite),
                ),
            ));
        });
}
