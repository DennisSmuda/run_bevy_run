use bevy::prelude::Color;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  Menu,
  InGame,
  // GameOver,
}
// Dimensions
pub const GAME_WIDTH: f32 = 720.;
pub const GAME_HEIGHT: f32 = 420.;

// Colors
pub const BG_COLOR: Color = Color::rgb(38. / 255., 20. / 255., 40. / 255.);
// pub const PLAYER_COLOR: Color = Color::rgb(255. / 255., 228. / 255., 120. / 255.);
// pub const ENEMY_COLOR: Color = Color::rgb(60. / 255., 163. / 255., 112. / 255.);
