use bevy::prelude::*;

mod constants;
use constants::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  Menu,
  InGame,
  GameOver,
}

fn main() {
  let mut app = App::new();
  // Window setup
  app
    .insert_resource(ClearColor(BG_COLOR))
    .insert_resource(WindowDescriptor {
      title: "Run Rust!".to_string(),
      width: GAME_WIDTH,
      height: GAME_HEIGHT,
      ..Default::default()
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins);

  // Startup system (cameras)
  app.add_startup_system(camera_setup);
  // Run the app
  app.run();
}

fn camera_setup(mut commands: Commands) {
  // 2D orthographic camera
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  // UI Camera
  commands.spawn_bundle(UiCameraBundle::default());
}
