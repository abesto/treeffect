use bevy::prelude::*;

use super::{
    events::SpawnPlayer,
    systems::{
        generic::{spawn, Spawn},
        player::spawn_player,
    },
};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn, spawn_player))
            .add_event::<SpawnPlayer>()
            .add_event::<Spawn>();
    }
}
