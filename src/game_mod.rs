use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::ScalingMode};
use chrono::*;
use colored::Colorize;
use rand::Rng;
use std::{io, process::exit};

use crate::AppState;

#[derive(Resource, Default)]
pub struct GameData {
    player_guess: String,
    green_tally: i32,
    red_tally: i32,
    result: String,
}

#[derive(Resource)]
pub struct MyTimer {
    pub timer: Timer,
}

pub fn game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_data: ResMut<GameData>,
) {
    game_data.green_tally = 0;
    game_data.red_tally = 0;
    let mut level = 1;
    let mut rng = rand::thread_rng();
    let mut input = String::new();
    // let (sender, receiver) = std::sync::mpsc::channel();

    let title = format!("Level {} - Which color has more tallies?", level);
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

    for _iter in 3..rng.gen_range(4..20) {
        let color = print_shapes(
            rng.gen_range(0..10),
            //rng.gen_range(0..10),
            &asset_server,
            &mut commands,
            _iter,
        );

        if color == 'g' {
            game_data.green_tally += 1;
        } else {
            game_data.red_tally += 1;
        }
    }

    next_state.set(AppState::Pause);
}

fn print_shapes(
    random: i32,
    //print_amt: i32,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    offset: i32,
) -> char {
    if random % 2 == 0 {
        //for _iter in 0..1 {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let pop_entity = commands.spawn({
            Text2dBundle {
                text: Text::from_section(
                    "X",
                    TextStyle {
                        font,
                        font_size: 24.0,
                        color: Color::GREEN,
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
        return 'g';
    } else {
        //for _iter in 0..print_amt {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        commands.spawn({
            Text2dBundle {
                text: Text::from_section(
                    "X",
                    TextStyle {
                        font,
                        font_size: 24.0,
                        color: Color::RED,
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
        return 'r';
    }
}

pub fn clear_shapes(mut commands: Commands, mut query: Query<Entity, With<Text>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

fn process_guess(
    guess: String,
    green: i32,
    red: i32,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
) -> bool {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    if guess.trim() == "g" && green > red {
        let correct_green_entity = commands.spawn({
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
        game_data.result = "Correct!".to_string();
        return true;
    } else if guess.trim() == "r" && red > green {
        let correct_red_entity = commands.spawn({
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
        game_data.result = "Correct!".to_string();
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
        game_data.result = "Incorrect!".to_string();
        return false;
    }
}

pub fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // Spawn Camera in Foreground
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ..default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                // bottom left button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
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
                background_color: Color::GREEN.into(),
                ..default()
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
            parent.spawn(ButtonBundle {
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
                background_color: Color::RED.into(),
                ..default()
            });
        });
}

pub fn game_over(mut commands: Commands, game_data: Res<GameData>) {
    //exit game.
}

pub fn interact_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // Buttons
    for (interaction, color, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = Color::WHITE.into();
                // if the background color of the button is green, return 'g'
                if color.0 == Color::GREEN {
                    game_data.player_guess = "g".to_string();
                } else {
                    game_data.player_guess = "r".to_string();
                }
                process_guess(
                    game_data.player_guess.clone(),
                    game_data.green_tally,
                    game_data.red_tally,
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
    if timer.timer.finished() && game_data.result == "Correct!" {
        next_state.set(AppState::StartGame);
        timer.timer.reset();
    } else if timer.timer.finished() && game_data.result == "Incorrect!" {
        next_state.set(AppState::GameOver);
    }
}
