use bevy::{
    ecs::bundle::Bundle,
    prelude::default,
    text::{TextSection, TextStyle},
    ui::{node_bundles::TextBundle, PositionType, Style},
};

use crate::{
    components,
    constants
};

#[derive(Bundle)]
pub struct ScoreboardBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    pub text_bundle: TextBundle,
    pub scoreboard: components::Scoreboard,
}

impl ScoreboardBundle {
    pub fn new(scoreboard_text: impl Into<String>) -> ScoreboardBundle {
        ScoreboardBundle {
            text_bundle: TextBundle::from_sections([
                TextSection::new(
                    scoreboard_text,
                    TextStyle {
                        font_size: constants::scoreboard::SCOREBOARD_FONT_SIZE,
                        color: constants::text::TEXT_COLOR,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: constants::scoreboard::SCOREBOARD_FONT_SIZE,
                    color: constants::scoreboard::SCORE_COLOR,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: constants::scoreboard::SCOREBOARD_TEXT_PADDING,
                left: constants::scoreboard::SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
            scoreboard: components::Scoreboard,
        }
    }
}
