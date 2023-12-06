use bevy::prelude::*;

use crate::{components::intents::wait::WaitIntent, events::took_turn::TookTurn};

fn wait(
    mut commands: Commands,
    mut query: Query<Entity, With<WaitIntent>>,
    mut ev_took_turn: EventWriter<TookTurn>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).remove::<WaitIntent>();
        ev_took_turn.send(entity.into());
    }
}

pub struct WaitPlugin;

impl Plugin for WaitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wait);
    }
}
