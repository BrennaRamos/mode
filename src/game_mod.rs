use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_tweening::{
    lens::{TransformRotationLens, UiPositionLens},
    Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween,
};
use rand::Rng;
use std::time::Duration;

use crate::{
    main_menu::{SoundEffect, BASIL_GREEN, OLIVE_GREEN},
    settings::GameSettings,
    AppState,
};

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

#[derive(Component, Clone, Debug, PartialEq)]
pub enum FruitType {
    Apple,
    Pear,
    Orange,
    Strawberry,
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

#[derive(Component)]
pub struct RoundTimer;

#[derive(Resource)]
pub struct VillagersGame {
    pub villagers: Vec<(String, bool, i32)>,
}

impl Default for VillagersGame {
    fn default() -> Self {
        Self {
            villagers: vec![
                ("characters/baker_solo.png".to_string(), false, 6),
                ("characters/bug_collector_solo.png".to_string(), false, 11),
                ("characters/traveler_solo.png".to_string(), false, 21),
                ("characters/farmer_solo.png".to_string(), false, 31),
                ("characters/gardener_solo.png".to_string(), false, 41),
                ("characters/librarian_solo.png".to_string(), false, 51),
                ("characters/merchant_solo.png".to_string(), false, 61),
                ("characters/penguin_solo.png".to_string(), false, 81),
                ("characters/student_solo.png".to_string(), false, 91),
                ("characters/cat_solo.png".to_string(), false, 101),
            ],
        }
    }
}

pub fn play_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
    game_settings: Res<GameSettings>,
    timer: Res<PauseTimer>,
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
    // Spawn Timer Text

    let timer: String = format!("{:?}", timer.pause_timer.remaining_secs());
    let font = asset_server.load("fonts/Leila-Regular.ttf");
    commands.spawn((
        {
            TextBundle {
                text: Text::from_section(
                    timer,
                    TextStyle {
                        font,
                        font_size: 48.0,
                        color: OLIVE_GREEN,
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    position_type: PositionType::Relative,
                    top: Val::Percent(2.0),
                    left: Val::Percent(1.0),

                    ..default()
                },
                ..default()
            }
        },
        RoundTimer,
    ));

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

// Spawn Villagers
pub fn spawn_chibi_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    villagers: Res<VillagersGame>,
) {
    let columns = 10;
    let rows = 1;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // Use the CSS Grid algorithm for laying out this node
                    display: Display::Grid,
                    // Make node fill the entirety it's parent (in this case the window)
                    width: Val::Percent(30.0),
                    height: Val::Percent(30.0),
                    // center the node vertically and horizontally within the window
                    position_type: PositionType::Relative,
                    top: Val::Px(450.0),
                    left: Val::Px(10.0),
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
                            aspect_ratio: Some(1.0),
                            // Use grid layout for this node
                            display: Display::Grid,
                            // Add 24px of padding around the grid
                            padding: UiRect::all(Val::Px(24.0)),
                            // Set the grid to have 10 columns all with sizes minmax(0, 1fr)
                            // This creates 10 exactly evenly sized columns
                            grid_template_columns: RepeatedGridTrack::flex(columns, 1.0),
                            // Set the grid to have 1 rows all with sizes minmax(0, 1fr)
                            // This creates 1 exactly evenly sized rows
                            grid_template_rows: RepeatedGridTrack::flex(rows, 1.0),
                            // Set a 12px gap/gutter between rows and columns
                            column_gap: Val::Px(122.0),
                            //left: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    GridIdentifier::Grid,
                ))
                .with_children(|builder| {
                    for (index, villager) in villagers.villagers.iter().enumerate() {
                        item_rect_villager(
                            builder,
                            &asset_server,
                            villager.0.clone(),
                            villager.1.clone(),
                            index % 2 == 0,
                        );
                    }
                });
        });
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
    villagers: &Res<VillagersGame>,
) -> bool {
    let font = asset_server.load("fonts/Leila-Regular.ttf");
    if (guess.trim() == "x" && exes > os) || (guess.trim() == "o" && os > exes) {
        // Spawn Correct
        commands.spawn((
            AudioBundle {
                source: asset_server.load("music/Correct.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..Default::default()
                },
                ..default()
            },
            SoundEffect,
        ));
        commands.spawn((
            {
                TextBundle {
                    text: Text::from_section(
                        "Correct!",
                        TextStyle {
                            font: font.clone(),
                            font_size: 48.0,
                            color: OLIVE_GREEN,
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
            },
            Animator::new(Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(500),
                UiPositionLens {
                    start: UiRect {
                        top: Val::Percent(70.0),
                        left: Val::Percent(43.0),

                        ..default()
                    },
                    end: UiRect {
                        top: Val::Percent(60.0),
                        left: Val::Percent(43.0),

                        ..default()
                    },
                },
            )),
        ));
        for villager in villagers.villagers.iter() {
            let level_unlockable = villager.2;
            let status = villager.1;
            if (game_data.level == level_unlockable) && status == false {
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("music/NewChar.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn,
                            ..Default::default()
                        },
                        ..default()
                    },
                    SoundEffect,
                ));
                commands.spawn((
                    {
                        TextBundle {
                            text: Text::from_section(
                                "New Villager Unlocked!",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 48.0,
                                    color: OLIVE_GREEN,
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            style: Style {
                                top: Val::Percent(75.0),
                                left: Val::Percent(33.0),

                                ..default()
                            },
                            ..default()
                        }
                    },
                    Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        Duration::from_millis(500),
                        UiPositionLens {
                            start: UiRect {
                                top: Val::Percent(75.0),
                                left: Val::Percent(33.0),

                                ..default()
                            },
                            end: UiRect {
                                top: Val::Percent(65.0),
                                left: Val::Percent(33.0),

                                ..default()
                            },
                        },
                    )),
                ));
            }
        }
        game_data.result = Result::Correct;
        return true;
    } else {
        //Spawn Incorrect
        commands.spawn((
            AudioBundle {
                source: asset_server.load("music/Incorrect.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..Default::default()
                },
                ..default()
            },
            SoundEffect,
        ));
        commands.spawn((
            {
                TextBundle {
                    text: Text::from_section(
                        "Incorrect!",
                        TextStyle {
                            font: font.clone(),
                            font_size: 48.0,
                            color: Color::CRIMSON,
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
            },
            Animator::new(Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(500),
                UiPositionLens {
                    start: UiRect {
                        top: Val::Percent(70.0),
                        left: Val::Percent(42.0),

                        ..default()
                    },
                    end: UiRect {
                        top: Val::Percent(60.0),
                        left: Val::Percent(42.0),

                        ..default()
                    },
                },
            )),
        ));
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

    // Spawn Music
    commands.spawn(AudioBundle {
        source: asset_server.load("music/Bees.ogg"),
        settings: PlaybackSettings {
            volume: Volume::new_relative(0.5),
            ..Default::default()
        },
        ..default()
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
    villagers: Res<VillagersGame>,
) {
    // Keyboard Input
    // Escape to Main Menu
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
        next_state.set(AppState::GameOver);
    }
    // Fruit A Select
    if keyboard_input.just_released(KeyCode::Z) {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("music/Answer.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..Default::default()
                },
                ..default()
            },
            SoundEffect,
        ));
        game_data.player_guess = "x".to_string();
        timer.pause_timer.pause();
        process_guess(
            game_data.player_guess.clone(),
            game_data.exes,
            game_data.os,
            &asset_server,
            &mut commands,
            &mut game_data,
            &villagers,
        );
        next_state.set(AppState::ShowResults);
    }
    // Fruit B Select
    if keyboard_input.just_released(KeyCode::X) {
        commands.spawn((
            AudioBundle {
                source: asset_server.load("music/Answer.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..Default::default()
                },
                ..default()
            },
            SoundEffect,
        ));
        game_data.player_guess = "o".to_string();
        timer.pause_timer.pause();
        process_guess(
            game_data.player_guess.clone(),
            game_data.exes,
            game_data.os,
            &asset_server,
            &mut commands,
            &mut game_data,
            &villagers,
        );
        next_state.set(AppState::ShowResults);
    }
    // UI Button Input
    for (interaction, answer_button, mut border_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("music/Answer.ogg"),
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
                    &villagers,
                );
                next_state.set(AppState::ShowResults);
            }
            Interaction::Hovered => {
                *border_color = Color::SALMON.into();
            }
            Interaction::None => {
                *border_color = BASIL_GREEN.into();
            }
        }
    }
}

pub fn show_results(
    time: Res<Time>,
    mut timer: ResMut<ResultTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    game_data: ResMut<GameData>,
    mut villagers: ResMut<VillagersGame>,
) {
    timer.result_timer.tick(time.delta());
    if timer.result_timer.finished() {
        match game_data.result {
            Result::Correct => {
                for villager in villagers.villagers.iter_mut() {
                    let level_unlockable = villager.2;
                    if game_data.level == level_unlockable {
                        villager.1 = true;
                    }
                }
                next_state.set(AppState::StartRound);
                timer.result_timer.reset();
            }
            Result::Incorrect => {
                next_state.set(AppState::GameOver);
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
    mut query_music: Query<Entity, (With<PlaybackSettings>, Without<SoundEffect>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
) {
    game_data.level = 1;
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
    for entity in query_music.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    next_state.set(AppState::MainMenu);
}

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
}

fn item_rect_villager(
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
                    padding: UiRect::all(Val::Px(3.0)),
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
                        Color::NONE.into()
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

pub fn update_timer(timer: Res<PauseTimer>, mut query_timer: Query<&mut Text, With<RoundTimer>>) {
    for mut text in query_timer.iter_mut() {
        text.sections[0].value = format!("{:.1}", timer.pause_timer.remaining_secs());
    }
}
