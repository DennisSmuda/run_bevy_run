use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// Player Component
#[derive(Component)]
pub struct Player {
    pub direction: MoveDirection,
    pub speed: f32,
    pub id: u32,
}

// Enemy Component
#[derive(Component)]
pub struct Enemy {
    pub direction: MoveDirection,
    pub speed: f32,
}

#[derive(Component)]
pub enum Collider {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Rotatable {
    pub speed: f32,
    pub direction: i8,
}

// MoveDirection Component
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MoveDirection {
    Left,
    Up,
    Right,
    Down,
    None,
}

///
/// Implement Distribution Trait to get a random direction
/// see => https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
impl Distribution<MoveDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveDirection {
        match rng.gen_range(0..=3) {
            // rand 0.8
            0 => MoveDirection::Up,
            1 => MoveDirection::Down,
            2 => MoveDirection::Right,
            _ => MoveDirection::Left,
        }
    }
}
