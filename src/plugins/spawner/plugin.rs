use bevy::prelude::*;

use super::events::SpawnPlayer;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, super::systems::player::spawn_player)
            .add_event::<SpawnPlayer>();
    }
}
