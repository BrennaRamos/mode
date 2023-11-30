use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
//use chrono::*;
//use colored::Colorize;
use rand::Rng;
//use std::{io, process::exit};

use crate::AppState;

#[derive(Resource)]
pub struct GameData {
    player_guess: String,
    exes: i32,
    os: i32,
    result: Result,
    level: i32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            player_guess: Default::default(),
            exes: Default::default(),
            os: Default::default(),
            result: Default::default(),
            level: 1,
        }
    }
}

#[derive(Resource)]
pub struct MyTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub enum AnswerButton {
    X,
    O,
}

#[derive(Component, Default)]
pub enum Result {
    #[default]
    Correct,
    Incorrect,
}

pub fn play_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
) {
    game_data.exes = 0;
    game_data.os = 0;
    let mut rng = rand::thread_rng();

    let title = format!("Level {} - Which color has more tallies?", game_data.level);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let level_entity = commands.spawn({
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
            transform: Transform::from_translation(Vec3::new(0.0, 60.0, 0.0)),
            ..default()
        }
    });

    let mut counter = 0;

    for _iter in 3..rng.gen_range(4..20) {
        let shape = print_shapes(
            rng.gen_range(1..11),
            //rng.gen_range(0..10),
            &asset_server,
            &mut commands,
            _iter,
        );

        match shape {
            AnswerButton::X => game_data.exes += 1,
            AnswerButton::O => game_data.os += 1,
        }
        counter = _iter + 1;
    }

    // To get rid of cases where there are equal amounts of either shape
    if game_data.exes == game_data.os {
        let shape = print_shapes(
            rng.gen_range(1..11),
            //rng.gen_range(0..10),
            &asset_server,
            &mut commands,
            counter,
        );

        match shape {
            AnswerButton::X => game_data.exes += 1,
            AnswerButton::O => game_data.os += 1,
        }
    }

    game_data.level += 1;
    next_state.set(AppState::Pause);
}

fn print_shapes(
    random: i32,
    //print_amt: i32,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    offset: i32,
) -> AnswerButton {
    if random % 2 == 0 {
        //for _iter in 0..1 {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let pop_entity = commands.spawn({
            Text2dBundle {
                text: Text::from_section(
                    "X",
                    TextStyle {
                        font,
                        font_size: 36.0,
                        color: Color::TEAL,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::new(
                    -200.0 + (offset * 20) as f32,
                    0.0,
                    0.0,
                )),
                ..default()
            }
        });
        //}
        return AnswerButton::X;
    } else {
        //for _iter in 0..print_amt {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        commands.spawn({
            Text2dBundle {
                text: Text::from_section(
                    "O",
                    TextStyle {
                        font,
                        font_size: 36.0,
                        color: Color::GOLD,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::new(
                    -200.0 + (offset * 20) as f32,
                    0.0,
                    0.0,
                )),
                ..default()
            }
        });
        //}
        return AnswerButton::O;
    }
}

fn process_guess(
    guess: String,
    exes: i32,
    os: i32,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
) -> bool {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    if guess.trim() == "x" && exes > os {
        let correct_x_entity = commands.spawn({
            Text2dBundle {
                text: Text::from_section(
                    "Correct!",
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: Color::GREEN,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
                ..default()
            }
        });
        game_data.result = Result::Correct;
        return true;
    } else if guess.trim() == "o" && os > exes {
        let correct_o_entity = commands.spawn({
            Text2dBundle {
                text: Text::from_section(
                    "Correct!",
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: Color::GREEN,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
                ..default()
            }
        });
        game_data.result = Result::Correct;
        return true;
    } else {
        let incorrect_entity = commands.spawn({
            Text2dBundle {
                text: Text::from_section(
                    "Incorrect!",
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: Color::RED,
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
                ..default()
            }
        });
        game_data.result = Result::Incorrect;
        return false;
    }
}

pub fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Spawn Camera in Foreground
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GRAY),
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..default()
    });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // bottom left button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            AnswerButton::X,
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
                        background_color: Color::TEAL.into(),
                        ..default()
                    },
                    AnswerButton::X,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "X",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ),
                        AnswerButton::X,
                    ));
                });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                // bottom right button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
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
                        background_color: Color::GOLD.into(),
                        ..default()
                    },
                    AnswerButton::O,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "O",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ),
                        AnswerButton::O,
                    ));
                });
        });
}

pub fn interact_button(
    mut interaction_query: Query<
        (&Interaction, &AnswerButton, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' currently pressed");
        game_data.player_guess = "x".to_string();
        process_guess(
            game_data.player_guess.clone(),
            game_data.exes,
            game_data.os,
            &asset_server,
            &mut commands,
            &mut game_data,
        );
        next_state.set(AppState::ShowResults);
    }

    if keyboard_input.just_released(KeyCode::D) {
        info!("'D' just pressed");
        game_data.player_guess = "o".to_string();
        process_guess(
            game_data.player_guess.clone(),
            game_data.exes,
            game_data.os,
            &asset_server,
            &mut commands,
            &mut game_data,
        );
        next_state.set(AppState::ShowResults);
    }
    // Buttons
    for (interaction, answer_button, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = Color::WHITE.into();
                match answer_button {
                    AnswerButton::X => game_data.player_guess = "x".to_string(),
                    AnswerButton::O => game_data.player_guess = "o".to_string(),
                }
                process_guess(
                    game_data.player_guess.clone(),
                    game_data.exes,
                    game_data.os,
                    &asset_server,
                    &mut commands,
                    &mut game_data,
                );
                next_state.set(AppState::ShowResults);
            }
            Interaction::Hovered => {
                *border_color = Color::BLACK.into();
            }
            Interaction::None => {
                *border_color = Color::WHITE.into();
            }
        }
    }
}

pub fn show_results(
    time: Res<Time>,
    mut timer: ResMut<MyTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    game_data: ResMut<GameData>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        match game_data.result {
            Result::Correct => {
                next_state.set(AppState::StartRound);
                timer.timer.reset();
            }
            Result::Incorrect => next_state.set(AppState::GameOver),
        }
    }
}

pub fn clear_shapes(
    mut commands: Commands,
    mut query: Query<Entity, (With<Text>, Without<AnswerButton>)>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn game_over(mut commands: Commands, mut query: Query<Entity>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
