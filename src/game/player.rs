use crate::*;

///
/// Player Movement
pub fn player_movement_system(mut players: Query<(&Player, &mut Transform)>) {
  let (player, mut transform) = players.single_mut();
  let translation = &mut transform.translation;
  // println!("Player Input System: {}", player.id);

  match &player.direction {
    Direction::Left => {
      translation.x -= player.speed * TIME_STEP;
    }
    Direction::Right => {
      translation.x += player.speed * TIME_STEP;
    }
    Direction::Up => {
      translation.y += player.speed * TIME_STEP;
    }
    Direction::Down => {
      translation.y -= player.speed * TIME_STEP;
    }
    Direction::None => {
      // println!("Stop")
    }
  }
  // Keep in bounds
  translation.x = translation.x.min(GAME_WIDTH / 2.).max(-GAME_WIDTH / 2.);
  translation.y = translation.y.min(GAME_HEIGHT / 2.).max(-GAME_HEIGHT / 2.);
}

///
/// Player Collisions
pub fn player_collision_system(
  mut state: ResMut<State<AppState>>,
  mut player_query: Query<(&mut Player, &Transform)>,
  collider_query: Query<(Entity, &Collider, &Transform)>,
) {
  let (_player, player_transform) = player_query.single_mut();
  let player_size = player_transform.scale.truncate();

  for (_collider_entity, _collider, transform) in collider_query.iter() {
    let collision = collide(
      player_transform.translation,
      player_size,
      transform.translation,
      transform.scale.truncate(),
    );
    if let Some(_collision) = collision {
      state.set(AppState::GameOver).unwrap();
    }
  }
}

///
/// Player Input System
pub fn player_input_system(
  mut state: ResMut<State<AppState>>,
  keyboard_input: Res<Input<KeyCode>>,
  mut players: Query<&mut Player>,
) {
  if let Some(mut player) = players.iter_mut().next() {
    // println!("Player Input System: {}", player.id);

    let dir: Direction = if keyboard_input.pressed(KeyCode::Up) | keyboard_input.pressed(KeyCode::K)
    {
      Direction::Up
    } else if keyboard_input.pressed(KeyCode::Down) | keyboard_input.pressed(KeyCode::J) {
      Direction::Down
    } else if keyboard_input.pressed(KeyCode::Left) | keyboard_input.pressed(KeyCode::H) {
      Direction::Left
    } else if keyboard_input.pressed(KeyCode::Right) | keyboard_input.pressed(KeyCode::L) {
      Direction::Right
    } else {
      Direction::None
    };

    player.direction = dir;
  }

  if keyboard_input.pressed(KeyCode::Escape) {
    state.set(AppState::Menu).unwrap();
  }
}

///
/// Spawn Player
pub fn spawn_player(mut commands: Commands) {
  // Spawn Player
  commands
    .spawn_bundle(SpriteBundle {
      transform: Transform {
        translation: Vec3::new(0.0, 0.0, 0.0),
        scale: Vec3::new(30.0, 30.0, 0.0),
        ..Default::default()
      },
      sprite: Sprite {
        color: PLAYER_COLOR,
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(Player {
      speed: 250.0,
      direction: Direction::Up,
      id: rand::random::<u32>(),
    })
    .insert(Collider::Player);
}
