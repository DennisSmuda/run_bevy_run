use crate::*;
use rand::prelude::*;

///
/// Move Enemies towards Direction
pub fn enemy_movement_system(
  mut commands: Commands,
  mut enemies: Query<(Entity, &Enemy, &mut Transform)>,
) {
  for (enemy_entity, enemy, mut transform) in enemies.iter_mut() {
    let translation = &mut transform.translation;
    match &enemy.direction {
      Direction::Left => {
        translation.x -= enemy.speed * TIME_STEP;
        if translation.x < (-GAME_WIDTH / 2.) - 16. {
          commands.entity(enemy_entity).despawn_recursive();
        }
      }
      Direction::Right => {
        translation.x += enemy.speed * TIME_STEP;
        if translation.x > (GAME_WIDTH / 2.) - 16. {
          commands.entity(enemy_entity).despawn_recursive();
        }
      }
      Direction::Up => {
        translation.y += enemy.speed * TIME_STEP;
        if translation.y > (GAME_HEIGHT / 2.) - 16. {
          commands.entity(enemy_entity).despawn_recursive();
        }
      }
      Direction::Down => {
        translation.y -= enemy.speed * TIME_STEP;
        if translation.y < (-GAME_HEIGHT / 2.) - 16. {
          commands.entity(enemy_entity).despawn_recursive();
        }
      }
      Direction::None => {
        translation.y -= enemy.speed * TIME_STEP;
      }
    }
  }
}

///
/// Spawn Enemy
/// Spawns on bounds and gets a random orthogonal direction
pub fn spawn_enemy_system(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
  if timer.0.tick(time.delta()).just_finished() {
    let direction: Direction = rand::random();
    let mut rng = rand::thread_rng();
    // random x/y + spawn-padding
    let mut x: f32 = rng.gen();
    let mut y: f32 = rng.gen();
    let spawn_padding: f32 = 0.8;
    let speed: f32 = rng.gen_range(32.0..264.0);

    // Spawn on opposite window-side to direction
    if direction == Direction::Left {
      x = GAME_WIDTH / 2.;
      y = rng.gen_range(-spawn_padding..spawn_padding) * GAME_HEIGHT / 2.;
    } else if direction == Direction::Right {
      x = -GAME_WIDTH / 2.;
      y = rng.gen_range(-spawn_padding..spawn_padding) * GAME_HEIGHT / 2.;
    }

    if direction == Direction::Up {
      y = -GAME_HEIGHT / 2.;
      x = rng.gen_range(-spawn_padding..spawn_padding) * GAME_WIDTH / 2.;
    } else if direction == Direction::Down {
      y = GAME_HEIGHT / 2.;
      x = rng.gen_range(-spawn_padding..spawn_padding) * GAME_WIDTH / 2.;
    }

    // Spawn Entity
    commands
      .spawn_bundle(SpriteBundle {
        transform: Transform {
          translation: Vec3::new(x, y, 0.0),
          scale: Vec3::new(24.0, 24.0, 0.0),
          ..Default::default()
        },
        sprite: Sprite {
          color: ENEMY_COLOR,
          ..Default::default()
        },
        ..Default::default()
      })
      .insert(Enemy {
        speed: speed,
        direction: direction,
      })
      .insert(Collider::Enemy)
      .id();
  }
}
