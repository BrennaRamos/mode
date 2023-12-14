use bevy::{asset::AssetMetaCheck, prelude::*};
mod game_mod;
mod how_to_play;
mod leaderboard;
mod loading;
mod main_menu;
mod settings;
use bevy_tweening::TweeningPlugin;
use game_mod::*;
use loading::LoadingTimer;
use settings::*;

#[derive(Resource, Default)]
pub struct Handles {
    pub audio_handles: Vec<Handle<AudioSource>>,
    pub image_handles: Vec<Handle<Image>>,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TweeningPlugin,
        ))
        .init_resource::<GameData>()
        .init_resource::<GameSettings>()
        .init_resource::<PlayerData>()
        .init_resource::<Handles>()
        .init_resource::<Villagers>()
        .add_state::<AppState>()
        .add_systems(Startup, startup)
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
            (
                how_to_play::interact_button,
                loading::animate_background_and_load,
            )
                .run_if(in_state(AppState::HowToPlay)),
        )
        .add_systems(
            Update,
            (
                leaderboard::interact_button,
                loading::animate_background_and_load,
            )
                .run_if(in_state(AppState::Leaderboard)),
        )
        .add_systems(
            Update,
            (
                settings::interact_button,
                settings::set_fruits,
                settings::hover_fruit,
                loading::animate_background_and_load,
            )
                .run_if(in_state(AppState::Settings)),
        )
        .add_systems(
            Update,
            (game_mod::show_results, loading::animate_background_and_load)
                .run_if(in_state(AppState::ShowResults)),
        )
        .insert_resource(ResultTimer {
            result_timer: Timer::from_seconds(0.5, TimerMode::Once),
        })
        .add_systems(
            Update,
            (
                game_mod::pause,
                game_mod::tick_elapsed,
                game_mod::interact_button,
                game_mod::update_timer,
                loading::animate_background_and_load,
            )
                .run_if(in_state(AppState::Pause)),
        )
        .insert_resource(PauseTimer {
            pause_timer: Timer::from_seconds(5.0, TimerMode::Once),
        })
        .insert_resource(LoadingTimer {
            loading_timer: Timer::from_seconds(4.0, TimerMode::Once),
        })
        .add_systems(OnEnter(AppState::MainMenu), main_menu::setup_menu)
        .add_systems(OnEnter(AppState::LoadingScreen), loading::setup_loading)
        .add_systems(OnEnter(AppState::Leaderboard), leaderboard::setup_ui)
        .add_systems(
            OnEnter(AppState::Settings),
            (
                settings::setup_ui,
                settings::spawn_chibi,
                settings::spawn_fruit,
            ),
        )
        .add_systems(OnEnter(AppState::HowToPlay), how_to_play::setup_ui)
        .add_systems(
            Update,
            (
                main_menu::animate_menu_title,
                main_menu::interact_menu,
                loading::animate_background_and_load,
            )
                .run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            (loading::animate_background_and_load, loading::tick_loading)
                .run_if(in_state(AppState::LoadingScreen)),
        )
        .add_systems(OnExit(AppState::MainMenu), main_menu::clear_shapes)
        .add_systems(OnExit(AppState::LoadingScreen), loading::clear_shapes)
        .add_systems(OnExit(AppState::HowToPlay), how_to_play::clear_shapes)
        .add_systems(OnExit(AppState::Leaderboard), leaderboard::clear_shapes)
        .add_systems(OnExit(AppState::Settings), settings::clear_shapes)
        .run();
}

fn startup(asset_server: Res<AssetServer>, mut handles: ResMut<Handles>) {
    handles
        .image_handles
        .push(asset_server.load("icons/Title.png"));
    handles
        .image_handles
        .push(asset_server.load("background/background.png"));
    handles
        .audio_handles
        .push(asset_server.load("music/Petunia.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Back.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Select.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Correct.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Incorrect.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Answer.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Bees.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Dandelions.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/NewChar.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Path.ogg"));
    handles
        .audio_handles
        .push(asset_server.load("music/Rain.ogg"));
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingScreen,
    MainMenu,
    StartRound,
    Pause,
    ShowResults,
    GameOver,
    QuitGame,
    Leaderboard,
    HowToPlay,
    Settings,
}
