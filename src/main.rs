use bevy::prelude::*;
mod game_mod;
use game_mod::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameData>()
        .add_systems(Startup, startup)
        .add_state::<AppState>()
        .add_systems(
            OnEnter(AppState::StartGame),
            (game_mod::clear_shapes, game_mod::game).chain(),
        )
        .add_systems(OnExit(AppState::GameOver), game_mod::game_over)
        .add_systems(Update, game_mod::interact_button)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_ui(&mut commands, &asset_server);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    StartGame,
    Pause,
    CheckAnswer,
    GameOver,
}
