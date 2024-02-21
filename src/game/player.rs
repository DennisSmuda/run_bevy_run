use crate::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

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
fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle { radius: 12.0 }));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(PLAYER_COLOR),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Player>,
) {
    if let Some(mut player) = players.iter_mut().next() {
        let dir: MoveDirection =
            if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyK) {
                MoveDirection::Up
            } else if keyboard_input.pressed(KeyCode::ArrowDown)
                || keyboard_input.pressed(KeyCode::KeyJ)
            {
                MoveDirection::Down
            } else if keyboard_input.pressed(KeyCode::ArrowLeft)
                || keyboard_input.pressed(KeyCode::KeyH)
            {
                MoveDirection::Left
            } else if keyboard_input.pressed(KeyCode::ArrowRight)
                || keyboard_input.pressed(KeyCode::KeyL)
            {
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

#[test]
fn setup_player_correctly() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<ColorMaterial>>();

    app.add_systems(Update, (setup_player).chain());
    app.update();

    let mut query = app.world.query::<&Player>();
    let player = query.iter(&app.world).next().unwrap();
    assert_eq!(player.speed, 250.0);
    assert_eq!(player.direction, MoveDirection::Up);
}
