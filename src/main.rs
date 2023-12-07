use bevy::prelude::*;
mod game_mod;
use game_mod::*;
mod main_menu;
use main_menu::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        .add_systems(Update, game_mod::interact_button)
        .add_systems(
            Update,
            main_menu::interact_menu.run_if(in_state(AppState::MainMenu)),
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
        .add_systems(
            OnExit(AppState::MainMenu),
            (main_menu::clear_shapes, game_mod::setup_ui),
        )
        .run();
}

fn startup(
    commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    setup_menu(commands, asset_server, camera_query);
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
}
