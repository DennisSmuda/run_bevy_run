use crate::*;

mod enemies;
mod game_ui;
mod player;

#[derive(Component)]
struct Velocity(Vec3);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerPlugin)
            .add_plugins(game_ui::GameUiPlugin)
            .add_plugins(enemies::EnemyPlugin);
    }
}
