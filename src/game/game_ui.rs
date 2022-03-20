use crate::*;

///
/// Spawn UI Bundle
pub fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
  // Score Text
  commands.spawn_bundle(TextBundle {
    style: Style {
      position_type: PositionType::Absolute,
      position: Rect {
        top: Val::Px(4.0),
        left: Val::Px(24.0),
        ..Default::default()
      },
      ..Default::default()
    },
    text: Text::with_section(
      format!("score: {} ", 0.),
      TextStyle {
        font: asset_server.load("fonts/Efforts.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
      },
      TextAlignment {
        horizontal: HorizontalAlign::Center,
        ..Default::default()
      },
    ),
    ..Default::default()
  });
}

///
/// Score system updates score on fixed interval
pub fn update_score_system(
  time: Res<Time>,
  mut timer: ResMut<ScoreTimer>,
  mut game_state: ResMut<GameState>,
  mut text_query: Query<&mut Text>,
) {
  if timer.0.tick(time.delta()).just_finished() {
    game_state.score += 10;
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("score: {}", game_state.score);
  }
}
