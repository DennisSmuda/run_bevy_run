use bevy::prelude::*;
use bevy::window::WindowResolution;

mod constants;
use constants::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}

const TIME_STEP: f32 = 5.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]

pub enum AppState {
    #[default]
    // Menu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating))) // Adjust spawn rate as needed
        .add_state::<AppState>()
        // .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Run Bevy".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(Update, spawn_enemies.run_if(in_state(AppState::InGame)))
        .add_systems(Update, move_enemies.run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_movement.run_if(in_state(AppState::InGame)))
        .add_systems(Update, check_collisions.run_if(in_state(AppState::InGame)))
        .add_systems(OnExit(AppState::InGame), teardown_game_state)
        .add_systems(OnEnter(AppState::GameOver), setup_gameover)
        .add_systems(Update, update_gameover.run_if(in_state(AppState::GameOver)))
        .add_systems(OnExit(AppState::GameOver), teardown_gameover_state)
        .run();
}

fn setup_gameover(mut commands: Commands) {
    let button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn update_gameover(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    let player_transform = player_query.single(); // Assumes there's exactly one player

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        if player_transform
            .translation
            .distance(enemy_transform.translation)
            < 30.0
        {
            // Collision detected, handle game over condition
            println!("Game Over!");
            // state.set(AppState::GameOver).unwrap();
            next_state.set(AppState::GameOver);
            // Here you might want to despawn the player, reset the game, or implement other game over logic
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

fn teardown_game_state(
    mut commands: Commands,
    entities: Query<Entity, With<Enemy>>,
    players: Query<Entity, With<Player>>,
) {
    println!("Tearing down state");
    for entity in entities.iter() {
        println!("Despawning entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
    for player in players.iter() {
        commands.entity(player).despawn_recursive();
    }
}

fn teardown_gameover_state(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}
