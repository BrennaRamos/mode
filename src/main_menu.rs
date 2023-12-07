use crate::AppState;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

#[derive(Component)]
pub enum ActionButton {
    Play,
    Quit,
}

pub fn start_game() {}

pub fn setup_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Spawn Camera in Foreground
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GRAY),
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..default()
    });
    // Spawn Title text
    let title = format!("Odemay");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn({
        Text2dBundle {
            text: Text::from_section(
                title,
                TextStyle {
                    font,
                    font_size: 64.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..default()
        }
    });
    // Spawn menu buttons
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
                                color: Color::BLACK,
                            },
                        ),
                        ActionButton::Play,
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
                                color: Color::BLACK,
                            },
                        ),
                        ActionButton::Quit,
                    ));
                });
        });
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
                    ActionButton::Play => next_state.set(AppState::StartRound),
                    ActionButton::Quit => next_state.set(AppState::QuitGame),
                }
            }
            Interaction::Hovered => {
                *border_color = Color::BLUE.into();
            }
            Interaction::None => {
                *border_color = Color::BLACK.into();
            }
        }
    }
}

pub fn clear_shapes(mut commands: Commands, mut query: Query<Entity, With<ActionButton>>) {
    for entity in query.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}

pub fn quit_game(mut commands: Commands, mut query: Query<Entity>) {
    for entity in query.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
