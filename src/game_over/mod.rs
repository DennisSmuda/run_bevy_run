use crate::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(setup_gameover_system))
      .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(update_gameover_system))
      .add_system_set(SystemSet::on_exit(AppState::GameOver).with_system(teardown_state));
  }
}

///
/// Setup Gameover
fn setup_gameover_system(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut game_state: ResMut<GameState>,
) {
  let font = asset_server.load("fonts/Efforts.ttf");

  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        margin: Rect::all(Val::Auto), // This will center the current node
        flex_direction: FlexDirection::ColumnReverse,
        align_items: AlignItems::Center,
        ..Default::default()
      },
      color: Color::rgba(1., 1., 1., 0.).into(),
      ..Default::default()
    })
    .with_children(|parent| {
      // Display two lines of text, the second one with the current settings
      parent.spawn_bundle(TextBundle {
        style: Style {
          // margin: Rect::all(Val::Px(50.0)),
          ..Default::default()
        },
        text: Text::with_section(
          "game over",
          TextStyle {
            font: font.clone(),
            font_size: 64.0,
            color: Color::WHITE,
          },
          Default::default(),
        ),
        ..Default::default()
      });
      parent.spawn_bundle(TextBundle {
        style: Style {
          margin: Rect::all(Val::Px(32.0)),
          ..Default::default()
        },
        text: Text::with_section(
          format!("you scored {} points!", game_state.score),
          TextStyle {
            font: font.clone(),
            font_size: 32.0,
            color: Color::WHITE,
          },
          Default::default(),
        ),
        ..Default::default()
      });

      parent
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
              "try again",
              TextStyle {
                font: font.clone(),
                font_size: 32.0,
                color: Color::WHITE,
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
    });

  // Reset Score!
  game_state.score = 0;
}

///
/// Update Gameover
fn update_gameover_system(
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
