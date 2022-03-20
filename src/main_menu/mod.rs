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
  // Play Button
  spawn_play_button(commands, asset_server);
}

///
/// Update Main Menu
fn update_menu_system(
  mut state: ResMut<State<AppState>>,
  mut interaction_query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
  for (interaction, mut color) in interaction_query.iter_mut() {
    match *interaction {
      Interaction::Clicked => {
        *color = PRESSED_BUTTON.into();
        state.set(AppState::InGame).unwrap();
      }
      Interaction::Hovered => {
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}

fn spawn_play_button(mut commands: Commands, asset_server: Res<AssetServer>) {
  // let font = asset_server.load("fonts/Efforts.ttf");
  commands
    .spawn_bundle(ButtonBundle {
      style: Style {
        size: Size::new(Val::Px(160.0), Val::Px(65.0)),
        margin: Rect::all(Val::Auto),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
      },
      color: NORMAL_BUTTON.into(),
      ..Default::default()
    })
    .with_children(|parent| {
      parent.spawn_bundle(TextBundle {
        text: Text::with_section(
          "Play",
          TextStyle {
            font: asset_server.load("fonts/Efforts.ttf"),
            font_size: 40.0,
            color: Color::rgb(1., 1., 1.),
          },
          TextAlignment {
            horizontal: HorizontalAlign::Right,
            vertical: VerticalAlign::Center,
          },
        ),
        style: Style {
          margin: Rect {
            left: Val::Px(0.),
            right: Val::Px(0.),
            top: Val::Px(0.),
            bottom: Val::Px(10.),
          },
          ..Default::default()
        },
        ..Default::default()
      });
    })
    .id();
}
