use crate::*;

mod game_ui;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Velocity(Vec3);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_game)
            .add_systems(OnEnter(AppState::InGame), game_ui::setup)
            .add_systems(Update, game_ui::update.run_if(in_state(AppState::InGame)))
            .add_systems(Update, spawn_enemies.run_if(in_state(AppState::InGame)))
            .add_systems(Update, move_enemies.run_if(in_state(AppState::InGame)))
            .add_systems(Update, player_movement.run_if(in_state(AppState::InGame)))
            .add_systems(Update, check_collisions.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), teardown)
            .add_systems(OnExit(AppState::InGame), game_ui::teardown);
    }
}

fn setup_game(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player);
}

fn spawn_enemies(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        // Determine spawn position and velocity here
        // For example, spawn on the left edge, moving right:
        let spawn_position = Vec3::new(-300.0, rand::random::<f32>() * 600.0 - 300.0, 0.0);
        let velocity = Vec3::new(10.0, 0.0, 0.0); // Adjust speed and direction as needed

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: ENEMY_COLOR,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(spawn_position),
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Velocity(velocity));
    }
}

fn move_enemies(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform), With<Enemy>>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn check_collisions(
    mut next_state: ResMut<NextState<AppState>>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let player_transform = player_query.single(); // Assumes there's exactly one player

    for (_enemy_entity, enemy_transform) in enemy_query.iter() {
        if player_transform
            .translation
            .distance(enemy_transform.translation)
            < 30.0
        {
            // Collision detected, handle game over condition
            next_state.set(AppState::GameOver);
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        transform.translation += direction * TIME_STEP;
    }
}

fn teardown(
    mut commands: Commands,
    entities: Query<Entity, With<Enemy>>,
    players: Query<Entity, With<Player>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for player in players.iter() {
        commands.entity(player).despawn_recursive();
    }
}
