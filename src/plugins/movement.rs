use bevy::prelude::*;

use crate::components::{intents::movement::MovementIntent, position::Position};

use super::map::resources::Map;

fn movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Position, &MovementIntent)>,
    map: Res<Map>,
) {
    for (entity, mut position, MovementIntent(direction)) in query.iter_mut() {
        let new_position = map.iclamp(&(position.xy.as_ivec2() + *direction));
        if map.is_walkable(&new_position) {
            position.xy = new_position;
        }
        commands.entity(entity).remove::<MovementIntent>();
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}
