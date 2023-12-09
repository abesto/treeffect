use bevy::prelude::*;
use bevy_eventlistener::prelude::*;

use crate::{
    behavior,
    components::{energy::Active, position::Position},
    events::{intents::movement::MovementIntent, took_turn::TookTurn},
    plugins::map::resources::Map,
};

fn movement(
    mut query: Query<&mut Position, With<Active>>,
    map: Res<Map>,
    intent: Listener<MovementIntent>,
    mut ev_took_turn: EventWriter<TookTurn>,
) {
    let entity = intent.listener();
    let Ok(mut position) = query.get_mut(entity) else {
        return;
    };
    let new_position = map.iclamp(&(position.xy.as_ivec2() + intent.vector));
    if map.is_walkable(&new_position) {
        position.xy = new_position;
    }
    ev_took_turn.send(entity.into());
}

behavior!(movement);
