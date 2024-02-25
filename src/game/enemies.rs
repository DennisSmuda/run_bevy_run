use crate::*;

use rand::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemies.run_if(in_state(AppState::InGame)))
            .add_systems(Update, move_enemies.run_if(in_state(AppState::InGame)))
            .add_systems(Update, rotate_enemies.run_if(in_state(AppState::InGame)))
            .add_systems(Update, check_collisions.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), teardown);
    }
}

fn spawn_enemies(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();

        let direction: MoveDirection = rand::random();
        let speed: f32 = rng.gen_range(32.0..264.0);

        let rotation_speed: f32 = rng.gen_range(0.2..0.3);
        let rotation_direction = rng.gen_range(-2..2);

        let mut x: f32 = rng.gen();
        let mut y: f32 = rng.gen();
        let spawn_padding: f32 = 0.8;

        // Spawn on opposite window-side to direction
        if direction == MoveDirection::Left {
            x = WINDOW_WIDTH / 2.;
            y = rng.gen_range(-spawn_padding..spawn_padding) * WINDOW_HEIGHT / 2.;
        } else if direction == MoveDirection::Right {
            x = -WINDOW_WIDTH / 2.;
            y = rng.gen_range(-spawn_padding..spawn_padding) * WINDOW_HEIGHT / 2.;
        }

        if direction == MoveDirection::Up {
            y = -WINDOW_HEIGHT / 2.;
            x = rng.gen_range(-spawn_padding..spawn_padding) * WINDOW_WIDTH / 2.;
        } else if direction == MoveDirection::Down {
            y = WINDOW_HEIGHT / 2.;
            x = rng.gen_range(-spawn_padding..spawn_padding) * WINDOW_WIDTH / 2.;
        }

        let spawn_position = Vec3::new(x, y, 0.);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: ENEMY_COLOR,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                transform: Transform::from_translation(spawn_position),
                ..default()
            })
            .insert(Enemy { speed, direction })
            .insert(Rotatable {
                speed: rotation_speed,
                direction: rotation_direction,
            })
            .insert(Collider::Enemy);
    }
}

pub fn rotate_enemies(mut enemies: Query<(Entity, &Enemy, &mut Transform, &Rotatable)>) {
    for (_enemy_entity, _enemy, mut transform, rotatable) in enemies.iter_mut() {
        transform.rotate_z(rotatable.speed * TIME_STEP * rotatable.direction as f32);
    }
}

pub fn move_enemies(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Enemy, &mut Transform)>,
    mut event_writer: EventWriter<EnemyKilledEvent>,
) {
    for (enemy_entity, enemy, mut transform) in enemies.iter_mut() {
        let translation = &mut transform.translation;
        match &enemy.direction {
            &MoveDirection::Left => {
                translation.x -= enemy.speed * TIME_STEP;
                if translation.x < (-WINDOW_WIDTH / 2.) - 16. {
                    commands.entity(enemy_entity).despawn_recursive();
                    event_writer.send(EnemyKilledEvent(enemy_entity));
                }
            }
            MoveDirection::Right => {
                translation.x += enemy.speed * TIME_STEP;
                if translation.x > (WINDOW_WIDTH / 2.) - 16. {
                    commands.entity(enemy_entity).despawn_recursive();
                    event_writer.send(EnemyKilledEvent(enemy_entity));
                }
            }
            MoveDirection::Up => {
                translation.y += enemy.speed * TIME_STEP;
                if translation.y > (WINDOW_HEIGHT / 2.) - 16. {
                    commands.entity(enemy_entity).despawn_recursive();
                    event_writer.send(EnemyKilledEvent(enemy_entity));
                }
            }
            MoveDirection::Down => {
                translation.y -= enemy.speed * TIME_STEP;
                if translation.y < (-WINDOW_HEIGHT / 2.) - 16. {
                    commands.entity(enemy_entity).despawn_recursive();
                    event_writer.send(EnemyKilledEvent(enemy_entity));
                }
            }
            MoveDirection::None => {
                translation.y -= enemy.speed * TIME_STEP;
            }
        }
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

fn teardown(mut commands: Commands, entities: Query<Entity, With<Enemy>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
