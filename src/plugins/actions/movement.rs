use bevy::prelude::*;

use crate::{
    components::{energy::Active, intents::movement::MovementIntent, position::Position},
    events::took_turn::TookTurn,
    plugins::map::resources::Map,
};

fn movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Position, &MovementIntent), With<Active>>,
    map: Res<Map>,
    mut ev_took_turn: EventWriter<TookTurn>,
) {
    for (entity, mut position, MovementIntent(direction)) in query.iter_mut() {
        let new_position = map.iclamp(&(position.xy.as_ivec2() + *direction));
        if map.is_walkable(&new_position) {
            position.xy = new_position;
        }
        commands.entity(entity).remove::<MovementIntent>();
        ev_took_turn.send(entity.into());
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}
