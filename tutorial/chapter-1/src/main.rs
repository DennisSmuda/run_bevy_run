use bevy::prelude::*;

mod constants;
use constants::*;

///
/// State Plugins
mod main_menu;
use main_menu::MainMenuPlugin;
mod game;
use game::GamePlugin;
// mod game_over;
// use game_over::GameOverPlugin;

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
    .add_plugins(DefaultPlugins)
    // States
    .add_plugin(MainMenuPlugin)
    .add_plugin(GamePlugin)
    .add_state(AppState::Menu);

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

pub fn teardown_state(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
  for entity in entities.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
