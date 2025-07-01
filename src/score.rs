use bevy::prelude::*;
use crate::TEXT_COLOR;

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
pub struct Score(usize);

#[derive(Component)]
pub struct ScoreboardUi;

impl Plugin for Score {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scoreboard)
            .add_systems(Update, update_scoreboard).insert_resource(Score(0));
    }
}

fn update_scoreboard(
    score: Res<crate::Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

fn setup_scoreboard(mut commands: Commands) {
    // Scoreboard
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        )],
    ));
}


