use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{
    main_menu::{SoundEffect, BASIL_GREEN, FONT, OLIVE_GREEN},
    settings::Villagers,
    AppState,
};

#[derive(Component)]
pub enum BackButton {
    MainMenu,
}

use bevy::input::keyboard::KeyboardInput;
use bevy_async_task::{AsyncTaskRunner, AsyncTaskStatus};
use serde::{Deserialize, Serialize};

pub struct LeaderboardPlugin;
impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    listen_received_character_events,
                    listen_keyboard_input_events,
                    update_text_edit_state,
                    update_button_look,
                    handle_submit_button,
                    spawn_leaderboard,
                )
                    .run_if(in_state(AppState::Leaderboard)),
                update_round,
            ),
        )
        .init_resource::<User>()
        .add_event::<UpdateRoundEvent>();
    }
}

#[derive(Event)]
pub struct UpdateRoundEvent {
    pub round: i64,
}

#[derive(Resource, Default, Deserialize, Serialize)]
pub struct User {
    pub user: String,
    pub pin: i64,
    pub score: i64,
}

#[derive(Component)]
pub struct Leaderboard;

#[derive(Component)]
pub struct LoginText;

#[derive(Component)]
pub struct LoginForm;

#[derive(Component)]
pub struct Username;

#[derive(Component)]
pub struct Password {
    pub value: String,
}

#[derive(Component)]
pub struct Submit;

#[derive(Component)]
pub struct Editing;

#[derive(Component)]
pub struct Editable;

#[derive(Serialize, Deserialize, Debug)]
struct UserScore {
    user: String,
    score: i64,
}

pub fn setup_scene(
    mut commands: Commands,
    login_data: ResMut<User>,
    asset_server: Res<AssetServer>,
) {
    if !login_data.user.is_empty() {
        spawn_user_text(&mut commands, &login_data, &asset_server);

        return;
    }
    // Spawn form buttons if user is default
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_self: JustifySelf::Start,
                    align_self: AlignSelf::End,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(25.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            },
            LoginForm,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text::from_section(
                    "USERNAME : min 3".to_string(),
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 20.0,
                        color: OLIVE_GREEN,
                        ..default()
                    },
                ),
                ..default()
            });
            builder
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(50.0),
                            width: Val::Px(400.0),
                            ..default()
                        },
                        ..default()
                    },
                    Username,
                    Editable,
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            "".to_string(),
                            TextStyle {
                                font: asset_server.load(FONT),
                                font_size: 40.0,
                                color: Color::BEIGE,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });

            builder.spawn(TextBundle {
                text: Text::from_section(
                    "PIN : only digits, min 4".to_string(),
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 20.0,
                        color: OLIVE_GREEN,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });
            builder
                .spawn((
                    ButtonBundle {
                        style: Style {
                            height: Val::Px(50.0),
                            width: Val::Px(400.0),
                            ..default()
                        },
                        ..default()
                    },
                    Password {
                        value: String::new(),
                    },
                    Editable,
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            "".to_string(),
                            TextStyle {
                                // font: asset_server.load(FONT),
                                font_size: 40.0,
                                color: Color::BEIGE,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });

            builder
                .spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(20.0)),
                            height: Val::Px(60.0),
                            width: Val::Px(200.0),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            justify_items: JustifyItems::Center,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Submit,
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle {
                        text: Text::from_section(
                            "SUBMIT".to_string(),
                            TextStyle {
                                font: asset_server.load(FONT),
                                font_size: 40.0,
                                color: Color::BEIGE,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn update_round(
    mut round_executor: AsyncTaskRunner<Result<ehttp::Response, ehttp::Error>>,
    mut update_round_events: EventReader<UpdateRoundEvent>,
    mut user: ResMut<User>,
) {
    match round_executor.poll() {
        AsyncTaskStatus::Finished(_) => (),
        _ => (),
    }

    for event in update_round_events.read() {
        if event.round > user.score {
            user.score = event.round;
            if let Ok(user) = serde_json::ser::to_vec(&User {
                user: user.user.clone(),
                pin: user.pin,
                score: user.score,
            }) {
                let mut request = ehttp::Request::post("https://ode.halyte.net/update", user);
                request
                    .headers
                    .insert("Content-Type".into(), "application/json".into());
                round_executor.start(ehttp::fetch_async(request));
            }
        }
    }
}

fn spawn_leaderboard(
    mut commands: Commands,
    mut response_executor: AsyncTaskRunner<Result<ehttp::Response, ehttp::Error>>,
    leaderboard_query: Query<Entity, With<Leaderboard>>,
    asset_server: Res<AssetServer>,
) {
    if !leaderboard_query.is_empty() {
        return;
    }

    match response_executor.poll() {
        AsyncTaskStatus::Idle => {
            let request = ehttp::Request::get("https://ode.halyte.net/");
            response_executor.start(ehttp::fetch_async(request));
        }
        AsyncTaskStatus::Finished(response) => {
            if let Ok(response) = response {
                if let Ok(leaderboard) =
                    serde_json::from_str::<Vec<UserScore>>(response.text().unwrap())
                {
                    commands
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    justify_self: JustifySelf::Center,
                                    align_self: AlignSelf::Center,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            Leaderboard,
                        ))
                        .with_children(|builder| {
                            for (idx, user_score) in leaderboard.iter().enumerate() {
                                builder
                                    .spawn(NodeBundle {
                                        style: Style {
                                            justify_content: JustifyContent::SpaceBetween,
                                            align_items: AlignItems::Center,
                                            width: Val::Px(300.0),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|builder| {
                                        builder.spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::right(Val::Px(25.0)),
                                                width: Val::Px(20.0),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                format!("{}", idx + 1),
                                                TextStyle {
                                                    font: asset_server.load(FONT),
                                                    font_size: 25.0,
                                                    color: OLIVE_GREEN,
                                                    ..default()
                                                },
                                            ),
                                            ..default()
                                        });
                                        builder.spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::right(Val::Px(5.0)),
                                                flex_grow: 0.0,
                                                flex_shrink: 0.0,
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                user_score.user.clone(),
                                                TextStyle {
                                                    font: asset_server.load(FONT),
                                                    font_size: 25.0,
                                                    color: OLIVE_GREEN,
                                                    ..default()
                                                },
                                            ),
                                            ..default()
                                        });
                                        builder.spawn(TextBundle {
                                            style: Style {
                                                margin: UiRect::left(Val::Auto),
                                                padding: UiRect::right(Val::Px(20.0)),
                                                ..default()
                                            },
                                            text: Text::from_section(
                                                format!("{}", user_score.score),
                                                TextStyle {
                                                    font: asset_server.load(FONT),
                                                    font_size: 25.0,
                                                    color: OLIVE_GREEN,
                                                    ..default()
                                                },
                                            ),
                                            ..default()
                                        });
                                    });
                            }
                        });
                }
            }
        }
        _ => (),
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserLogin {
    user: String,
    pin: i64,
}

pub fn cleanup(
    mut commands: Commands,
    leaderboard_query: Query<Entity, With<Leaderboard>>,
    login_form_query: Query<Entity, With<LoginForm>>,
    login_text_query: Query<Entity, With<LoginText>>,
) {
    for entity in leaderboard_query
        .iter()
        .chain(login_form_query.iter())
        .chain(login_text_query.iter())
    {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}

fn handle_submit_button(
    mut commands: Commands,
    login_form_query: Query<Entity, With<LoginForm>>,
    submit_button_query: Query<&Interaction, (Changed<Interaction>, With<Submit>)>,
    username_button_query: Query<Entity, With<Username>>,
    password_button_query: Query<&Password>,
    children_query: Query<&Children>,
    text_query: Query<&Text>,
    editing_query: Query<Entity, With<Editing>>,
    mut submit_executor: AsyncTaskRunner<Result<ehttp::Response, ehttp::Error>>,
    mut login_data: ResMut<User>,
    mut villagers: ResMut<Villagers>,
    asset_server: Res<AssetServer>,
    leaderboard_query: Query<Entity, With<Leaderboard>>,
) {
    match submit_executor.poll() {
        AsyncTaskStatus::Finished(result) => {
            if let Ok(result) = result {
                if let Ok(user) = serde_json::from_slice::<User>(&result.bytes) {
                    // Despawn form
                    for login_entity in login_form_query.iter().chain(leaderboard_query.iter()) {
                        if let Some(login_entity) = commands.get_entity(login_entity) {
                            login_entity.despawn_recursive();
                        }
                    }

                    // Write resource
                    login_data.user = user.user;
                    login_data.pin = user.pin;
                    login_data.score = user.score;

                    for (_index, villager) in villagers.villagers.iter_mut().enumerate() {
                        if user.score >= (villager.3 - 1) as i64 {
                            villager.2 = true;
                        }
                    }

                    spawn_user_text(&mut commands, &login_data, &asset_server);
                }
            }
        }
        _ => (),
    }

    for button_interaction in submit_button_query.iter() {
        match *button_interaction {
            Interaction::Pressed => {
                for editing_entity in editing_query.iter() {
                    if let Some(mut editing_entity) = commands.get_entity(editing_entity) {
                        editing_entity.remove::<Editing>();
                    }

                    let username_button = username_button_query.single();
                    let password_button = password_button_query.single();
                    let mut username = None;
                    let mut password = None;
                    for child in children_query.iter_descendants(username_button) {
                        if let Ok(text) = text_query.get(child) {
                            if text.sections[0].value.len() > 2 {
                                username = Some(text.sections[0].value.clone());
                            }
                        }
                    }

                    if password_button.value.len() > 3 {
                        password = Some(password_button.value.clone());
                    }

                    if let Some(username) = username {
                        if let Some(password) = password {
                            if let Ok(pin) = password.parse::<i64>() {
                                let new_user = User {
                                    user: username,
                                    pin,
                                    score: login_data.score,
                                };
                                if let Ok(new_user) = serde_json::ser::to_vec(&new_user) {
                                    let mut request = ehttp::Request::post(
                                        "https://ode.halyte.net/user",
                                        new_user,
                                    );
                                    request
                                        .headers
                                        .insert("Content-Type".into(), "application/json".into());
                                    submit_executor.start(ehttp::fetch_async(request));
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

/// Spawn text with username/score
fn spawn_user_text(commands: &mut Commands, user: &ResMut<User>, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_self: JustifySelf::Start,
                    align_self: AlignSelf::End,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::left(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            LoginText,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text::from_section(
                    format!("Logged in as: {}", user.user),
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 40.0,
                        color: OLIVE_GREEN,
                        ..default()
                    },
                ),
                style: Style { ..default() },
                ..default()
            });
            builder.spawn(TextBundle {
                text: Text::from_section(
                    format!("Personal Best: {}", user.score),
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 30.0,
                        color: OLIVE_GREEN,
                        ..default()
                    },
                ),
                style: Style { ..default() },
                ..default()
            });
        });
}

fn update_button_look(
    mut button_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<Editable>),
    >,
    mut submit_button_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<Submit>, Without<Editable>),
    >,
) {
    for (mut button_background, button_interaction) in button_query.iter_mut() {
        match *button_interaction {
            Interaction::Pressed => button_background.0 = BASIL_GREEN,
            Interaction::Hovered => button_background.0 = Color::SALMON,
            Interaction::None => button_background.0 = OLIVE_GREEN,
        }
    }
    for (mut button_background, button_interaction) in submit_button_query.iter_mut() {
        match *button_interaction {
            Interaction::Pressed => button_background.0 = OLIVE_GREEN,
            Interaction::Hovered => button_background.0 = Color::SALMON,
            Interaction::None => button_background.0 = BASIL_GREEN,
        }
    }
}

fn update_text_edit_state(
    mut commands: Commands,
    button_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Editable>)>,
    editing_query: Query<Entity, With<Editing>>,
    input: Res<Input<KeyCode>>,
) {
    for (button_entity, button_interaction) in button_query.iter() {
        match *button_interaction {
            Interaction::Pressed => {
                for editing_entity in editing_query.iter() {
                    if let Some(mut editing_entity) = commands.get_entity(editing_entity) {
                        editing_entity.remove::<Editing>();
                    }
                }

                if let Some(mut text_entity) = commands.get_entity(button_entity) {
                    text_entity.insert(Editing);
                }
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }

    if input.just_pressed(KeyCode::Escape) {
        for editing_entity in editing_query.iter() {
            if let Some(mut editing_entity) = commands.get_entity(editing_entity) {
                editing_entity.remove::<Editing>();
            }
        }
    }
}

fn listen_received_character_events(
    mut events: EventReader<ReceivedCharacter>,
    mut button_query: Query<(Entity, Option<&mut Password>), With<Editing>>,
    mut text_query: Query<&mut Text>,
    children_query: Query<&Children>,
) {
    for event in events.read() {
        for (button_entity, password) in button_query.iter_mut() {
            if password.is_none() {
                for child in children_query.iter_descendants(button_entity) {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.sections[0].value.push(event.char);
                    }
                }
            } else if let Some(mut password) = password {
                if event.char.is_digit(10) {
                    password.value.push(event.char);
                    for child in children_query.iter_descendants(button_entity) {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            text.sections[0].value.push('*');
                        }
                    }
                }
            }
        }
    }
}

fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    mut button_query: Query<(Entity, Option<&mut Password>), With<Editing>>,
    mut text_query: Query<&mut Text>,
    children_query: Query<&Children>,
) {
    for event in events.read() {
        match event.key_code {
            Some(KeyCode::Back) => {
                for (button_entity, password) in button_query.iter_mut() {
                    for child in children_query.iter_descendants(button_entity) {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            text.sections[0].value.pop();
                        }
                    }
                    if let Some(mut password) = password {
                        password.value.pop();
                    }
                }
            }
            _ => continue,
        }
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Music
    commands.spawn(AudioBundle {
        source: asset_server.load("music/Petunia.ogg"),
        settings: PlaybackSettings {
            volume: Volume::new_relative(0.5),
            ..Default::default()
        },
        ..default()
    });
    // Spawn Title Text
    let title = format!("Leaderboard");

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
                top: Val::Vh(-40.0),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        }
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
    mut query_stamps: Query<Entity, With<Sprite>>,
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
    for entity in query_music.iter_mut() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
