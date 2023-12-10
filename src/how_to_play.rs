use crate::AppState;
use bevy::prelude::*;
use bevy::render::color::*;

const OLIVE_GREEN: Color = Color::rgb(82.0 / 255.0, 88.0 / 255.0, 32.0 / 255.0);

#[derive(Component)]
pub enum BackButton {
    MainMenu,
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Title Text
    let title = format!("How To Play");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn({
        Text2dBundle {
            text: Text::from_section(
                title,
                TextStyle {
                    font,
                    font_size: 64.0,
                    color: OLIVE_GREEN,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(Vec3::new(0.0, 150.0, 0.0)),
            ..default()
        }
    });

    // Spawn Subtitle Text
    let title = format!("It's a fast-paced guessing game! The rules are simple:");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn({
        Text2dBundle {
            text: Text::from_section(
                title,
                TextStyle {
                    font,
                    font_size: 32.0,
                    color: OLIVE_GREEN,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(Vec3::new(0.0, 60.0, 0.0)),
            ..default()
        }
    });

    // Spawn Bullet Point Text
    let title = format!("    • You must guess which fruit appears most frequently on the screen. The mode, if you will...
    • Click the corresponding button/key to submit your answer
    • Each round gives you five seconds to guess. Guess as quickly as you can for a better score!
    • If you guess incorrectly, the game is over and you go back to the main menu
    • Depending on your performance, you can unlock more characters to dance with you :)");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn({
        Text2dBundle {
            text: Text::from_section(
                title,
                TextStyle {
                    font,
                    font_size: 32.0,
                    color: OLIVE_GREEN,
                },
            )
            .with_alignment(TextAlignment::Left),
            transform: Transform::from_translation(Vec3::new(0.0, -80.0, 0.0)),
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
                            "Back to Main Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 28.0,
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

pub fn clear_shapes(
    mut commands: Commands,
    mut query: Query<Entity, With<BackButton>>,
    mut query_title: Query<Entity, With<Text>>,
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
}
