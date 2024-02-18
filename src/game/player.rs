use crate::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_player)
            .add_systems(Update, player_input.run_if(in_state(AppState::InGame)))
            .add_systems(Update, player_movement.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), teardown);
    }
}

///
/// Player Setup
///
fn setup_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            speed: 250.0,
            direction: MoveDirection::Up,
            id: rand::random::<u32>(),
        })
        .insert(Collider::Player);
}

///
/// Player Movement
///
fn player_movement(mut players: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = players.single_mut();
    let translation = &mut transform.translation;
    // println!("Player Input System: {}", player.id);

    match &player.direction {
        &MoveDirection::Left => {
            translation.x -= player.speed * TIME_STEP;
        }
        MoveDirection::Right => {
            translation.x += player.speed * TIME_STEP;
        }
        MoveDirection::Up => {
            translation.y += player.speed * TIME_STEP;
        }
        MoveDirection::Down => {
            translation.y -= player.speed * TIME_STEP;
        }
        MoveDirection::None => {
            // println!("Stop")
        }
    }
    // Keep in bounds
    translation.x = translation.x.min(WINDOW_WIDTH / 2.).max(-WINDOW_WIDTH / 2.);
    translation.y = translation
        .y
        .min(WINDOW_HEIGHT / 2.)
        .max(-WINDOW_HEIGHT / 2.);
}

///
/// Player Input
///
pub fn player_input(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut Player>,
) {
    if let Some(mut player) = players.iter_mut().next() {
        let dir: MoveDirection =
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::K) {
                MoveDirection::Up
            } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::J) {
                MoveDirection::Down
            } else if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::H) {
                MoveDirection::Left
            } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::L) {
                MoveDirection::Right
            } else {
                MoveDirection::None
            };

        player.direction = dir;
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        next_state.set(AppState::GameOver)
    }
}

fn teardown(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in players.iter() {
        commands.entity(player).despawn_recursive();
    }
}
