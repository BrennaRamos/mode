use crate::AppState;
use bevy::render::color::*;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Level Text
    let title = format!("How To Play");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn({
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
}
