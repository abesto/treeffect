use bevy::prelude::*;

use crate::{
    components::{ai::Ai, energy::Active, player::Player, position::Position},
    events::intents::{attack::AttackIntent, movement::MovementIntent, wait::WaitIntent},
};

use super::map::resources::Map;

macro_rules! skip {
    ($ev:ident, $entity:ident) => {
        $ev.send(WaitIntent { actor: $entity });
        continue;
    };
}

fn dog(
    q_ai: Query<(Entity, &Position, &Ai), With<Active>>,
    q_player: Query<&Position, With<Player>>,
    mut ev_movement: EventWriter<MovementIntent>,
    mut ev_wait: EventWriter<WaitIntent>,
    map: Res<Map>,
) {
    for (entity, position, ai) in q_ai.iter() {
        if ai != &Ai::Dog {
            continue;
        }

        let Ok(player_pos) = q_player.get_single() else {
            skip!(ev_wait, entity);
        };

        let path = map.astar(&position.xy, &player_pos.xy);
        let Some((steps, _)) = path else {
            skip!(ev_wait, entity);
        };
        let Some(step) = steps.get(1) else {
            skip!(ev_wait, entity);
        };

        let from = position.xy.as_ivec2();
        let to = step.as_ivec2();
        let direction = if player_pos.xy.as_ivec2().distance_squared(from) > 4 {
            to - from
        } else {
            from - to
        };
        ev_movement.send(MovementIntent {
            actor: entity,
            vector: direction,
        });
    }
}

fn monster(
    q_ai: Query<(Entity, &Position, &Ai), With<Active>>,
    q_player: Query<&Position, With<Player>>,
    mut ev_movement: EventWriter<MovementIntent>,
    mut ev_attack: EventWriter<AttackIntent>,
    mut ev_wait: EventWriter<WaitIntent>,
    map: Res<Map>,
) {
    for (entity, position, ai) in q_ai.iter() {
        if ai != &Ai::Monster {
            continue;
        }

        let Ok(player_pos) = q_player.get_single() else {
            skip!(ev_wait, entity);
        };

        if player_pos
            .xy
            .as_ivec2()
            .distance_squared(position.xy.as_ivec2())
            > 100
        {
            skip!(ev_wait, entity);
        }

        let path = map.astar(&position.xy, &player_pos.xy);
        let Some((steps, _)) = path else {
            skip!(ev_wait, entity);
        };
        let Some(step) = steps.get(1) else {
            skip!(ev_wait, entity);
        };

        let from = position.xy.as_ivec2();
        let to = step.as_ivec2();
        let direction = to - from;

        if player_pos.xy.as_ivec2().distance_squared(from) < 4 {
            ev_attack.send(AttackIntent {
                actor: entity,
                vector: direction,
            });
        } else {
            ev_movement.send(MovementIntent {
                actor: entity,
                vector: direction,
            });
        }
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
