use bevy::prelude::Color;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const GAME_WIDTH: f32 = 720.;
pub const GAME_HEIGHT: f32 = 420.;

// App State => Screens/Rooms/Scenes
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
  Menu,
  InGame,
  GameOver,
}

// Game State => resource to hold score information
#[derive(Default)]
pub struct GameState {
  pub score: u64,
}

///
/// Colors
pub const BG_COLOR: Color = Color::rgb(38. / 255., 20. / 255., 40. / 255.);
pub const PLAYER_COLOR: Color = Color::rgb(255. / 255., 228. / 255., 120. / 255.);
pub const ENEMY_COLOR: Color = Color::rgb(26. / 255., 159. / 255., 222. / 255.);
// pub const GREEN_COLOR: Color = Color::rgb(60. / 255., 163. / 255., 112. / 255.);
// Button Colors
pub const NORMAL_BUTTON: Color = Color::rgb(128. / 255., 54. / 255., 107. / 255.);
pub const HOVERED_BUTTON: Color = Color::rgb(189. / 255., 72. / 255., 130. / 255.);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
