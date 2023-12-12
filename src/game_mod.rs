use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::{main_menu::OLIVE_GREEN, settings::GameSettings, AppState};

#[derive(Resource)]
pub struct GameData {
    player_guess: String,
    exes: i32,
    os: i32,
    result: Result,
    level: i32,
    time_elapsed: Duration,
    fruit_array: Vec<FruitType>,
    file_array: Vec<String>,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub enum FruitType {
    Apple,
    Pear,
    Orange,
    Strawberry,
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
            file_array: Default::default(),
            fruit_array: Default::default(),
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

#[derive(Component)]
pub enum GridIdentifier {
    Grid,
}

pub fn play_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
    game_settings: Res<GameSettings>,
) {
    game_data.fruit_array.clear();
    game_data.file_array.clear();
    game_data.player_guess.clear();
    game_data.time_elapsed = Duration::new(0, 0);
    game_data.exes = 0;
    game_data.os = 0;
    let mut rng = rand::thread_rng();

    // Spawn Level Text
    let title = format!("Level {}", game_data.level);
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
                position_type: PositionType::Relative,
                top: Val::Percent(10.0),
                left: Val::Percent(46.0),

                ..default()
            },
            ..default()
        }
    });

    let fruit_file_a = format!("icons/{:?}.png", game_settings.fruit_a).to_lowercase();
    let fruit_file_b = format!("icons/{:?}.png", game_settings.fruit_b).to_lowercase();

    for _iter in 3..rng.gen_range(4..20) {
        let shape = choose_fruits(rng.gen_range(1..11));

        match shape {
            AnswerButton::X => {
                game_data.exes += 1;
                game_data.fruit_array.push(game_settings.fruit_a.clone());
                game_data.file_array.push(fruit_file_a.clone());
            }
            AnswerButton::O => {
                game_data.os += 1;
                game_data.fruit_array.push(game_settings.fruit_b.clone());
                game_data.file_array.push(fruit_file_b.clone());
            }
        }
    }

    // To get rid of cases where there are equal amounts of either shape
    if game_data.exes == game_data.os {
        let shape = choose_fruits(rng.gen_range(1..11));

        match shape {
            AnswerButton::X => {
                game_data.exes += 1;
                game_data.fruit_array.push(game_settings.fruit_a.clone());
                game_data.file_array.push(fruit_file_a);
            }

            AnswerButton::O => {
                game_data.os += 1;
                game_data.fruit_array.push(game_settings.fruit_b.clone());
                game_data.file_array.push(fruit_file_b);
            }
        }
    }

    print_fruits(
        &asset_server,
        &mut commands,
        &game_data.fruit_array,
        &game_data.file_array,
    );
    game_data.level += 1;
    next_state.set(AppState::Pause);
}

fn choose_fruits(random: i32) -> AnswerButton {
    let return_answer: AnswerButton;

    if random % 2 == 0 {
        return_answer = AnswerButton::X;
    } else {
        return_answer = AnswerButton::O;
    }

    return return_answer;
}

fn print_fruits(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    fruit_array: &Vec<FruitType>,
    file_array: &Vec<String>,
) {
    let columns = 10;
    let rows = 5;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // Use the CSS Grid algorithm for laying out this node
                    display: Display::Grid,
                    // Make node fill the entirety it's parent (in this case the window)
                    width: Val::Percent(50.0),
                    height: Val::Percent(50.0),
                    // center the node vertically and horizontally within the window
                    position_type: PositionType::Relative,
                    top: Val::Percent(30.0),
                    left: Val::Percent(20.0),
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
                            // Make the height of the node fill its parent
                            height: Val::Percent(100.0),
                            // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                            // As the height is set explicitly, this means the width will adjust to match the height
                            aspect_ratio: Some(2.3),
                            // Use grid layout for this node
                            display: Display::Grid,
                            // Add 24px of padding around the grid
                            padding: UiRect::all(Val::Px(24.0)),
                            // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                            // This creates 4 exactly evenly sized columns
                            grid_template_columns: RepeatedGridTrack::flex(columns, 1.0),
                            // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                            // This creates 4 exactly evenly sized rows
                            grid_template_rows: RepeatedGridTrack::flex(rows, 1.0),
                            // Set a 12px gap/gutter between rows and columns
                            row_gap: Val::Px(15.0),
                            column_gap: Val::Px(15.0),
                            ..default()
                        },
                        // background_color: BackgroundColor(Color::BEIGE),
                        ..default()
                    },
                    GridIdentifier::Grid,
                ))
                .with_children(|builder| {
                    for (index, _fruit) in fruit_array.iter().enumerate() {
                        item_rect(
                            builder,
                            asset_server,
                            file_array[index].clone(),
                            fruit_array[index].clone(),
                        );
                    }
                });
        });
}

fn process_guess(
    guess: String,
    exes: i32,
    os: i32,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    game_data: &mut ResMut<GameData>,
) -> bool {
    let font = asset_server.load("fonts/Leila-Regular.ttf");
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
        game_data.result = Result::Incorrect;
        return false;
    }
}

pub fn setup_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_settings: &Res<GameSettings>,
) {
    let fruit_file_a = format!("icons/{:?}.png", game_settings.fruit_a).to_lowercase();
    let fruit_file_b = format!("icons/{:?}.png", game_settings.fruit_b).to_lowercase();

    // Spawn Timer Text
    let timer: String = format!("Time: 05:00");
    let font = asset_server.load("fonts/Leila-Regular.ttf");
    commands.spawn({
        TextBundle {
            text: Text::from_section(
                timer,
                TextStyle {
                    font,
                    font_size: 64.0,
                    color: OLIVE_GREEN,
                },
            )
            .with_alignment(TextAlignment::Center),
            style: Style {
                top: Val::Px(0.0),
                right: Val::Px(0.0),

                ..default()
            },
            ..default()
        }
    });

    commands
        .spawn((NodeBundle {
            style: Style {
                // bottom left button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        },))
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
                    game_settings.fruit_a.clone(),
                    AnswerButton::X,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage {
                                texture: asset_server.load(fruit_file_a),
                                ..default()
                            },
                            ..default()
                        },
                        game_settings.fruit_a.clone(),
                        AnswerButton::X,
                    ));
                });
        });

    commands
        .spawn((NodeBundle {
            style: Style {
                // bottom right button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        },))
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
                    game_settings.fruit_b.clone(),
                    AnswerButton::O,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            image: UiImage {
                                texture: asset_server.load(fruit_file_b),
                                ..default()
                            },
                            ..default()
                        },
                        game_settings.fruit_b.clone(),
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
    mut query_grid: Query<Entity, With<GridIdentifier>>,
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

    for entity in query_grid.iter_mut() {
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

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
fn item_rect(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    fruit_file: String,
    fruit_type: FruitType,
) {
    builder
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                // background_color: BackgroundColor(Color::BEIGE),
                ..default()
            },
            GridIdentifier::Grid,
        ))
        .with_children(|builder| {
            builder.spawn((
                ImageBundle {
                    image: asset_server.load(fruit_file).into(),
                    ..default()
                },
                fruit_type.clone(),
                GridIdentifier::Grid,
            ));
        });
}

pub fn spawn_timer(timer: ResMut<PauseTimer>) {}
