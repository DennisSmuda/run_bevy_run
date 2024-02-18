use bevy::prelude::*;
use bevy::window::WindowResolution;

mod constants;
use constants::*;

mod gameover;
use gameover::GameOverPlugin;

mod game;
use game::GamePlugin;

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
        .add_plugins(GameOverPlugin)
        .add_plugins(GamePlugin)
        .run();
}
