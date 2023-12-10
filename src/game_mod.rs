use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::{main_menu::OLIVE_GREEN, AppState};

#[derive(Resource)]
pub struct GameData {
    player_guess: String,
    exes: i32,
    os: i32,
    result: Result,
    level: i32,
    time_elapsed: Duration,
}

#[derive(Component)]
pub enum FruitType {
    Apple,
    Pear,
    Orange,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            player_guess: Default::default(),
            exes: Default::default(),
            os: Default::default(),
            result: Default::default(),
            level: 1,
            time_elapsed: Duration::new(0, 0),
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerData {
    username: String,
    pin: i32,
    level_reached: i32,
    speed: f32,
}

#[derive(Resource)]
pub struct ResultTimer {
    pub result_timer: Timer,
}

#[derive(Resource)]
pub struct PauseTimer {
    pub pause_timer: Timer,
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
    game_data.player_guess.clear();
    game_data.time_elapsed = Duration::new(0, 0);
    game_data.exes = 0;
    game_data.os = 0;
    let mut rng = rand::thread_rng();

    // Spawn Level Text
    let title = format!("Level {}", game_data.level);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
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
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("icons/apple.png"),
                transform: Transform::from_translation(Vec3::new(
                    -200.0 + (offset * 50) as f32,
                    0.0,
                    0.0,
                )),
                ..default()
            },
            FruitType::Apple,
        ));
        //}
        return AnswerButton::X;
    } else {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("icons/pear.png"),
                transform: Transform::from_translation(Vec3::new(
                    -200.0 + (offset * 50) as f32,
                    0.0,
                    0.0,
                )),
                ..default()
            },
            FruitType::Pear,
        ));
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
        // Spawn Correct
        commands.spawn({
            TextBundle {
                text: Text::from_section(
                    "Correct!",
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: Color::GREEN,
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    top: Val::Percent(70.0),
                    left: Val::Percent(43.0),

                    ..default()
                },
                ..default()
            }
        });
        game_data.result = Result::Correct;
        return true;
    } else if guess.trim() == "o" && os > exes {
        // Spawn Correct
        commands.spawn({
            TextBundle {
                text: Text::from_section(
                    "Correct!",
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: Color::GREEN,
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    top: Val::Percent(70.0),
                    left: Val::Percent(43.0),

                    ..default()
                },
                ..default()
            }
        });
        game_data.result = Result::Correct;
        return true;
    } else {
        //Spawn Incorrect
        commands.spawn({
            TextBundle {
                text: Text::from_section(
                    "Incorrect!",
                    TextStyle {
                        font: font.clone(),
                        font_size: 48.0,
                        color: Color::RED,
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    top: Val::Percent(70.0),
                    left: Val::Percent(42.0),

                    ..default()
                },
                ..default()
            }
        });
        //Spawn Tally of Fruits
        commands.spawn({
            TextBundle {
                text: Text::from_section(
                    "Tally of Fruits",
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: Color::RED,
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    top: Val::Percent(80.0),
                    left: Val::Percent(42.0),

                    ..default()
                },
                ..default()
            }
        });
        game_data.result = Result::Incorrect;
        return false;
    }
}

pub fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
                        ..default()
                    },
                    AnswerButton::X,
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
                        ..default()
                    },
                    AnswerButton::O,
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
    mut timer: ResMut<PauseTimer>,
) {
    // Keyboard Input
    if keyboard_input.just_released(KeyCode::Z) {
        game_data.player_guess = "x".to_string();
        timer.pause_timer.pause();
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

    if keyboard_input.just_released(KeyCode::X) {
        game_data.player_guess = "o".to_string();
        timer.pause_timer.pause();
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
    // UI Button Input
    for (interaction, answer_button, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *border_color = Color::WHITE.into();
                match answer_button {
                    AnswerButton::X => game_data.player_guess = "x".to_string(),
                    AnswerButton::O => game_data.player_guess = "o".to_string(),
                }
                timer.pause_timer.pause();
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
                *border_color = Color::SALMON.into();
            }
            Interaction::None => {
                *border_color = Color::rgb(82.0 / 255.0, 88.0 / 255.0, 32.0 / 255.0).into();
            }
        }
    }
}

pub fn show_results(
    time: Res<Time>,
    mut timer: ResMut<ResultTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
) {
    timer.result_timer.tick(time.delta());
    if timer.result_timer.finished() {
        match game_data.result {
            Result::Correct => {
                next_state.set(AppState::StartRound);
                timer.result_timer.reset();
            }
            Result::Incorrect => {
                next_state.set(AppState::GameOver);
                game_data.level = 1;
                timer.result_timer.reset();
            }
        }
    }
}

pub fn tick_elapsed(mut game_data: ResMut<GameData>, time: Res<Time>) {
    game_data.time_elapsed += time.delta();
}

pub fn pause(
    time: Res<Time>,
    mut timer: ResMut<PauseTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
) {
    timer.pause_timer.tick(time.delta());

    if timer.pause_timer.finished() {
        if game_data.player_guess.is_empty() {
            next_state.set(AppState::GameOver);
            timer.pause_timer.reset();
        }
    }
    if timer.pause_timer.paused() {
        timer.pause_timer.unpause();
        timer.pause_timer.reset();
        game_data.player_guess.clear();
    }
}

pub fn upload_score(game_data: ResMut<GameData>, mut player_data: ResMut<PlayerData>) {
    player_data.username = "sampleName".into();
    player_data.pin = 1234;
    player_data.level_reached = game_data.level;
    player_data.speed = game_data.time_elapsed.as_secs_f32();

    println!(
        "{} {} {} {:?}",
        player_data.username, player_data.pin, player_data.level_reached, player_data.speed
    );
}

pub fn clear_shapes(
    mut commands: Commands,
    mut query_text: Query<Entity, (With<Text>, Without<AnswerButton>)>,
    mut query_fruit: Query<Entity, (With<FruitType>, Without<Node>)>,
) {
    for entity in query_text.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }

    for entity in query_fruit.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}

pub fn game_over(
    mut commands: Commands,
    mut query_text: Query<Entity, With<Text>>,
    mut query_button: Query<Entity, With<Style>>,
    mut query_fruit: Query<Entity, With<FruitType>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for entity in query_text.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    for entity in query_button.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }

    for entity in query_fruit.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    next_state.set(AppState::MainMenu);
}
