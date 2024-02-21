use bevy::window::WindowResolution;
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

mod constants;
use constants::*;

mod components;
use components::*;

mod game;
use game::GamePlugin;

mod gameover;
use gameover::GameOverPlugin;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct ScoreTimer(Timer);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    GameOver,
}

// Game State => resource to hold score information
#[derive(Resource)]
pub struct GameState {
    pub score: u64,
}

fn main() {
    App::new()
        // Resources
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(GameState { score: 0 })
        // State
        .init_state::<AppState>()
        // Plugins
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
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
}
