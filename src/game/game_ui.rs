use crate::*;

///
/// Spawn UI Bundle
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Score Text
    commands.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        text: Text::from_section(
            format!("score: {} ", 0.),
            TextStyle {
                font: asset_server.load("fonts/Efforts.ttf"),
                font_size: 32.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    });
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
        text.sections[0].value = format!("score: {}", game_state.score);
        println!("Score: {}", game_state.score);
    }
}

pub fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
