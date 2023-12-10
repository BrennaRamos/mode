use bevy::{asset::AssetMetaCheck, prelude::*};
mod game_mod;
mod how_to_play;
mod leaderboard;
mod main_menu;
use game_mod::*;
use how_to_play::*;
use leaderboard::*;
use main_menu::*;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<GameData>()
        .init_resource::<PlayerData>()
        .add_systems(Startup, startup)
        .add_state::<AppState>()
        .add_systems(
            OnEnter(AppState::StartRound),
            (game_mod::clear_shapes, game_mod::play_game).chain(),
        )
        .add_systems(
            OnEnter(AppState::GameOver),
            (game_mod::game_over, game_mod::upload_score),
        )
        .add_systems(OnEnter(AppState::QuitGame), main_menu::quit_game)
        .add_systems(
            Update,
            game_mod::interact_button.run_if(in_state(AppState::Pause)),
        )
        .add_systems(
            Update,
            main_menu::interact_menu.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            how_to_play::interact_button.run_if(in_state(AppState::HowToPlay)),
        )
        .add_systems(
            Update,
            (leaderboard::interact_button, leaderboard::spawn_chibi)
                .run_if(in_state(AppState::Leaderboard)),
        )
        .add_systems(
            Update,
            game_mod::show_results.run_if(in_state(AppState::ShowResults)),
        )
        .insert_resource(ResultTimer {
            result_timer: Timer::from_seconds(0.5, TimerMode::Once),
        })
        .add_systems(
            Update,
            (game_mod::pause, game_mod::tick_elapsed).run_if(in_state(AppState::Pause)),
        )
        .insert_resource(PauseTimer {
            pause_timer: Timer::from_seconds(5.0, TimerMode::Once),
        })
        .add_systems(OnEnter(AppState::MainMenu), main_menu::setup_menu)
        .add_systems(OnEnter(AppState::Leaderboard), leaderboard::setup_ui)
        .add_systems(OnEnter(AppState::HowToPlay), how_to_play::setup_ui)
        .add_systems(
            Update,
            (main_menu::animate_menu_title).run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), main_menu::clear_shapes)
        .add_systems(OnExit(AppState::HowToPlay), how_to_play::clear_shapes)
        .add_systems(OnExit(AppState::Leaderboard), leaderboard::clear_shapes)
        .run();
}

fn startup(
    commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera2d>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    setup_menu(commands, asset_server, camera_query, texture_atlases);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    StartRound,
    Pause,
    ShowResults,
    GameOver,
    QuitGame,
    Leaderboard,
    HowToPlay,
}
