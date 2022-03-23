use crate::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game_system))
      .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update_game_system))
      .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(teardown_state));
  }
}

///
/// Setup Main Menu
fn setup_game_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  // Headline
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
      format!("Game!!!"),
      TextStyle {
        font: asset_server.load("fonts/Efforts.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
      },
      TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
      },
    ),
    ..Default::default()
  });
}

///
/// Update Main Menu
fn update_game_system(mut state: ResMut<State<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
  if keyboard_input.just_released(KeyCode::Escape) {
    state.set(AppState::Menu).unwrap();
  }
}
