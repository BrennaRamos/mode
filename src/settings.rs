use bevy::prelude::*;

use crate::{game_mod::FruitType, main_menu::OLIVE_GREEN, AppState};

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

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Title Text
    let title = format!("Settings");
    let font = asset_server.load("fonts/Leila-Regular.ttf");
    commands.spawn({
        TextBundle {
            text: Text::from_section(
                title,
                TextStyle {
                    font,
                    font_size: 64.0,
                    color: OLIVE_GREEN,
                },
            )
            .with_alignment(TextAlignment::Center),
            style: Style {
                top: Val::Percent(10.0),
                left: Val::Percent(42.0),

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
    mut interaction_query: Query<
        (&Interaction, &BackButton, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
    // Buttons
    for (interaction, answer_button, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
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
        (With<Button>),
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
                        *border_color = Color::CRIMSON.into();
                    } else if *fruit_type == game_settings.fruit_b {
                        *border_color = Color::BLUE.into();
                    } else {
                        *border_color = OLIVE_GREEN.into();
                    }
                    commands.entity(entity).remove::<Hovered>();
                    commands.entity(entity).remove::<Pressed>();
                }
            }
        } else if *interaction == Interaction::None {
            if *fruit_type == game_settings.fruit_a {
                *border_color = Color::CRIMSON.into();
            } else if *fruit_type == game_settings.fruit_b {
                *border_color = Color::BLUE.into();
            } else {
                *border_color = OLIVE_GREEN.into();
            }
            commands.entity(entity).remove::<Hovered>();
        }
    }
}

pub fn set_fruits(
    mut interaction_query: Query<&FruitType, (With<Button>, With<Hovered>)>,
    mouse_input: Res<Input<MouseButton>>,
    mut game_settings: ResMut<GameSettings>,
) {
    if mouse_input.just_released(MouseButton::Left) {
        for fruit_type in interaction_query.iter_mut() {
            if fruit_type != &game_settings.fruit_b {
                game_settings.fruit_a = fruit_type.clone();
            }
        }
    }
    if mouse_input.just_released(MouseButton::Right) {
        for fruit_type in interaction_query.iter_mut() {
            if fruit_type != &game_settings.fruit_a {
                game_settings.fruit_b = fruit_type.clone();
            }
        }
    }
}

pub fn spawn_chibi(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("characters/baker.png"),
        transform: Transform::from_xyz(0.0, -200.0, 0.0).with_scale(Vec3 {
            x: 3.0,
            y: 3.0,
            z: 1.0,
        }),
        ..default()
    });
}

pub fn spawn_fruit(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // bottom left button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    top: Val::Percent(40.0),
                    left: Val::Percent(15.0),
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
                            left: Val::Px(100.0),
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
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
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // bottom right button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    top: Val::Percent(40.0),
                    left: Val::Percent(30.0),
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
                            left: Val::Px(100.0),
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
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
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // bottom left button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    top: Val::Percent(40.0),
                    left: Val::Percent(45.0),
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
                            left: Val::Px(100.0),
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
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
                    ));
                });
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // bottom left button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    top: Val::Percent(40.0),
                    left: Val::Percent(60.0),
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
                            left: Val::Px(100.0),
                            border: UiRect {
                                top: Val::Px(2.),
                                left: Val::Px(2.),
                                bottom: Val::Px(2.),
                                right: Val::Px(2.),
                            },
                            ..default()
                        },
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
                    ));
                });
        });
}

pub fn clear_shapes(
    mut commands: Commands,
    mut query: Query<Entity, With<BackButton>>,
    mut query_title: Query<Entity, With<Text>>,
    mut query_stamps: Query<Entity, With<Sprite>>,
    mut query_fruits: Query<Entity, With<Node>>,
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
}
