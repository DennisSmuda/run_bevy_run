use bevy::prelude::*;
use rand::{
  distributions::{Distribution, Standard},
  Rng,
};

// Player Component
#[derive(Component)]
pub struct Player {
  pub direction: Direction,
  pub speed: f32,
  pub id: u32,
}

// Enemy Component
#[derive(Component)]
pub struct Enemy {
  pub direction: Direction,
  pub speed: f32,
}

#[derive(Component)]
pub enum Collider {
  Player,
  Enemy,
}

// Score Component
#[derive(Component)]
pub struct Score {
  pub value: usize,
}

// Direction Component
#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
  Left,
  Up,
  Right,
  Down,
  None,
}

///
/// Implement Distribution Trait to get a random direction
/// see => https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
impl Distribution<Direction> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
    match rng.gen_range(0..=3) {
      // rand 0.8
      0 => Direction::Up,
      1 => Direction::Down,
      2 => Direction::Right,
      _ => Direction::Left,
    }
  }
}
