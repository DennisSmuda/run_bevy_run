use crate::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (
                    update.run_if(in_state(AppState::InGame)),
                    on_enemy_killed.run_if(on_event::<EnemyKilledEvent>()),
                ),
            )
            .add_systems(OnExit(AppState::InGame), teardown);
    }
}

#[derive(Resource)]
pub struct GameUiData {
    text_entity: Entity,
}
///
/// Spawn UI Bundle
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Score Text
    let text_entity = commands
        .spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(12.),
                top: Val::Px(4.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            text: Text::from_section(
                format!("score: {} - enemies {}", 0., 0.),
                TextStyle {
                    font: asset_server.load("fonts/Efforts.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        })
        .id();
    commands.insert_resource(GameUiData { text_entity });
}

///
/// Score system updates score on fixed interval
pub fn update(
    time: Res<Time>,
    mut timer: ResMut<ScoreTimer>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        game_state.score += 10;
        let mut text = text_query.single_mut();
        text.sections[0].value = format!(
            "score: {} - enemies: {}",
            game_state.score, game_state.enemies_killed
        );
    }
}

pub fn on_enemy_killed(
    mut events: EventReader<EnemyKilledEvent>,
    mut game_state: ResMut<GameState>,
    mut text_query: Query<&mut Text>,
) {
    for _enemy_killed in events.read() {
        let mut text = text_query.single_mut();
        game_state.enemies_killed += 1;
        game_state.score += 5;
        text.sections[0].value = format!(
            "score: {} - enemies: {}",
            game_state.score, game_state.enemies_killed
        );
    }
}

pub fn teardown(mut commands: Commands, game_ui_data: Res<GameUiData>) {
    commands
        .entity(game_ui_data.text_entity)
        .despawn_recursive();
}
