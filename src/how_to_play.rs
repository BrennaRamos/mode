use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{
    main_menu::{SoundEffect, FONT, OLIVE_GREEN},
    AppState,
};

#[derive(Component)]
pub enum BackButton {
    MainMenu,
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Music
    commands.spawn(AudioBundle {
        source: asset_server.load("music/Path.ogg"),
        settings: PlaybackSettings {
            volume: Volume::new_relative(0.5),
            ..Default::default()
        },
        ..default()
    });
    // Spawn Title Text
    let title = format!("How To Play");

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

    let subtitle = format!(
        "The village of Odemay is hungry! They need more fruit to survive the winter. 
    Could you lend a hand in picking the fruit that is more abundant?"
    );
    let text = format!(
        "    .:. Two types of fruit will spawn on the screen in different quantities .:.
    .:. Choose which fruit there is more of using the Z and X keys or the on screen buttons .:.
    .:. Each level gives you five seconds to guess. Guess as quickly as you can for a better score! .:.
    .:. If you guess incorrectly, the game is over and you go back to the main menu .:.
    .:. Depending on your performance, you can unlock more characters for your village .:."
    );

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
            parent
                // Spawn Subtitle Text
                .spawn({
                    TextBundle {
                        text: Text::from_section(
                            subtitle,
                            TextStyle {
                                font: asset_server.load(FONT),
                                font_size: 32.0,
                                color: Color::SALMON,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        style: Style {
                            //top: Val::Vh(-10.0),
                            justify_self: JustifySelf::Center,
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        ..default()
                    }
                });
            // Spawn Text
            parent.spawn({
                TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: asset_server.load(FONT),
                            font_size: 27.0,
                            color: OLIVE_GREEN,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    style: Style {
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Vh(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                }
            });
        });

    // Spawn Menu Button
    commands
        .spawn((
            NodeBundle {
                style: Style {
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
    asset_server: Res<AssetServer>,
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

pub fn clear_shapes(
    mut commands: Commands,
    mut query: Query<Entity, With<BackButton>>,
    mut query_title: Query<Entity, With<Text>>,
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
    for entity in query_music.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
