use bevy::prelude::*;

use crate::components::{
    ai::Ai,
    energy::Active,
    intents::{movement::MovementIntent, wait::WaitIntent},
    player::Player,
    position::Position,
};

use super::map::resources::Map;

macro_rules! skip {
    ($ec:ident) => {
        $ec.insert(WaitIntent);
        continue;
    };
}

fn dog(
    mut commands: Commands,
    q_ai: Query<(Entity, &Position, &Ai), With<Active>>,
    q_player: Query<&Position, With<Player>>,
    map: Res<Map>,
) {
    for (entity, position, ai) in q_ai.iter() {
        if ai != &Ai::Dog {
            continue;
        }
        let mut entity_commands = commands.entity(entity);

        let Ok(player_pos) = q_player.get_single() else {
            skip!(entity_commands);
        };

        let path = map.astar(&position.xy, &player_pos.xy);
        let Some((steps, _)) = path else {
            skip!(entity_commands);
        };
        let Some(step) = steps.get(1) else {
            skip!(entity_commands);
        };

        let from = position.xy.as_ivec2();
        let to = step.as_ivec2();
        let direction = if player_pos.xy.as_ivec2().distance_squared(from) > 4 {
            to - from
        } else {
            from - to
        };
        entity_commands.insert(MovementIntent(direction));
    }
}

fn monster(
    mut commands: Commands,
    q_ai: Query<(Entity, &Position, &Ai), With<Active>>,
    q_player: Query<&Position, With<Player>>,
    map: Res<Map>,
) {
    for (entity, position, ai) in q_ai.iter() {
        if ai != &Ai::Monster {
            continue;
        }
        let mut entity_commands = commands.entity(entity);

        let Ok(player_pos) = q_player.get_single() else {
            skip!(entity_commands);
        };

        if player_pos
            .xy
            .as_ivec2()
            .distance_squared(position.xy.as_ivec2())
            > 100
        {
            skip!(entity_commands);
        }

        let path = map.astar(&position.xy, &player_pos.xy);
        let Some((steps, _)) = path else {
            skip!(entity_commands);
        };
        let Some(step) = steps.get(1) else {
            skip!(entity_commands);
        };

        let from = position.xy.as_ivec2();
        let to = step.as_ivec2();
        let direction = to - from;

        entity_commands.insert(MovementIntent(direction));
    }
}

pub fn ai_active(q_ai: Query<&Ai, With<Active>>) -> bool {
    q_ai.iter().next().is_some()
}

pub fn ai_inactive(q_ai: Query<&Ai, With<Active>>) -> bool {
    q_ai.iter().next().is_none()
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (dog, monster).run_if(ai_active));
    }
}
