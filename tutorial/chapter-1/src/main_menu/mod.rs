use crate::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu_system))
      .add_system_set(SystemSet::on_update(AppState::Menu).with_system(update_menu_system))
      .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(teardown_state));
  }
}

///
/// Setup Main Menu
fn setup_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
      format!("Run in Rust"),
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
fn update_menu_system(mut state: ResMut<State<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
  if keyboard_input.just_released(KeyCode::Space) {
    state.set(AppState::InGame).unwrap();
  }
}
