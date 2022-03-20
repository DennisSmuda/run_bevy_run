use crate::*;

mod enemies;
mod game_ui;
mod player;

pub struct GamePlugin;
///
/// Game Plugin
impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      // Timer Resources
      .insert_resource(ScoreTimer(Timer::from_seconds(2.0, true)))
      .insert_resource(SpawnTimer(Timer::from_seconds(0.8, true)))
      .add_system_set(
        SystemSet::on_enter(AppState::InGame)
          .with_system(game_ui::build_ui)
          .with_system(player::spawn_player),
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(player::player_input_system)
          .with_system(player::player_collision_system)
          .with_system(player::player_movement_system)
          .with_system(enemies::enemy_movement_system),
      )
      .add_system_set(
        SystemSet::on_update(AppState::InGame)
          .with_system(game_ui::update_score_system)
          .with_system(enemies::spawn_enemy_system),
      )
      .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(teardown_state));
  }
}
