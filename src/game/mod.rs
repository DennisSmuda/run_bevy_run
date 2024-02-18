use crate::*;

mod game_ui;
mod player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Velocity(Vec3);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_plugins(game_ui::GameUiPlugin)
            .add_systems(Update, spawn_enemies.run_if(in_state(AppState::InGame)))
            .add_systems(Update, move_enemies.run_if(in_state(AppState::InGame)))
            .add_systems(Update, check_collisions.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), teardown);
    }
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

fn teardown(mut commands: Commands, entities: Query<Entity, With<Enemy>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
