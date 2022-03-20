use bevy::{prelude::*, sprite::collide_aabb::collide};
// use bevy_inspector_egui::WorldInspectorPlugin;

///
/// Components and Constants
mod constants;
use constants::*;
mod components;
pub use components::Direction;
pub use components::*;

///
/// State Plugins
mod main_menu;
use main_menu::MainMenuPlugin;
mod game;
use game::GamePlugin;
mod game_over;
use game_over::GameOverPlugin;

pub struct ScoreTimer(Timer);
pub struct SpawnTimer(Timer);

///
/// App Entry
/// -> Setup window, plugins, resources
fn main() {
  let mut app = App::new();
  app
    .insert_resource(ClearColor(BG_COLOR))
    .insert_resource(WindowDescriptor {
      title: "Run in Rust".to_string(),
      width: 720.,
      height: 420.,
      ..Default::default()
    })
    // Resources
    .insert_resource(GameState { score: 0 })
    // Plugins
    .add_plugins(DefaultPlugins)
    // .add_plugin(WorldInspectorPlugin::new())
    // States
    .add_plugin(MainMenuPlugin)
    .add_plugin(GamePlugin)
    .add_plugin(GameOverPlugin)
    // Start in MenuState
    .add_state(AppState::Menu);

  // Startup system (cameras)
  app.add_startup_system(camera_setup);
  // Run the app
  app.run();
}

///
/// Spawn Cameras
fn camera_setup(mut commands: Commands) {
  // 2D orthographic camera
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  // UI Camera
  commands.spawn_bundle(UiCameraBundle::default());
}

///
/// Teardown State
/// gets called `on_exit` and deletes all entities but cameras
fn teardown_state(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
  for entity in entities.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
