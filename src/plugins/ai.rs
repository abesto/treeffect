use bevy::prelude::*;

use crate::components::{
    ai::Ai,
    energy::Active,
    intents::{movement::MovementIntent, wait::WaitIntent},
    player::Player,
    position::Position,
};

use super::map::resources::Map;

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
            entity_commands.insert(WaitIntent);
            continue;
        };

        print!(
            "Dog at {:?} is chasing player at {:?}",
            position.xy, player_pos.xy
        );
        let path = map.astar(&position.xy, &player_pos.xy);
        let Some((steps, _)) = path else {
            println!("No path found");
            entity_commands.insert(WaitIntent);
            continue;
        };
        let Some(step) = steps.get(1) else {
            println!("No steps found");
            entity_commands.insert(WaitIntent);
            continue;
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

fn ai_active(q_ai: Query<&Ai, With<Active>>) -> bool {
    q_ai.iter().next().is_some()
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (dog).run_if(ai_active));
    }
}
