use bevy::prelude::*;

use crate::{
    bundles::PlayerBundle,
    components::{player::Player, position::Position},
};

#[derive(Event)]
pub struct SpawnPlayer {
    pub xy: UVec2,
}

pub fn spawn_player(
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    mut ev_spawn_player: EventReader<SpawnPlayer>,
) {
    for ev in ev_spawn_player.read() {
        if let Ok(entity) = q_player.get_single() {
            commands.entity(entity).insert(Position::from(ev.xy));
        } else {
            commands.spawn(PlayerBundle {
                position: ev.xy.into(),
                ..Default::default()
            });

            #[cfg(feature = "wizard-mode")]
            add_wizard_items(commands, player_entity)
        }
    }
}
