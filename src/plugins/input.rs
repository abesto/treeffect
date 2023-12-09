use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::{ai::Ai, player::Player, position::Position},
    events::intents::{attack::AttackIntent, movement::MovementIntent},
};

use super::ai::ai_inactive;

const KEY_REPEAT: f64 = 0.050;
const KEY_REPEAT_DELAY: f64 = 0.400;

#[derive(Resource)]
struct KeyRepeatManager {
    keys: HashMap<KeyCode, f64>,
}

impl KeyRepeatManager {
    fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    fn pressed(&mut self, key: &KeyCode, dt: f64) -> bool {
        let first_press = self.keys.get(key).is_none();
        let held_duration = self.keys.get(key).map(|&t| dt + t).unwrap_or(dt);
        self.keys.insert(*key, held_duration);

        if held_duration < KEY_REPEAT_DELAY {
            return first_press;
        }
        (held_duration / KEY_REPEAT).round() > ((held_duration - dt) / KEY_REPEAT).round()
    }

    fn released(&mut self, key: &KeyCode) {
        self.keys.remove(key);
    }
}

fn handle_input(
    q_player: Query<(Entity, &Position), With<Player>>,
    q_monsters: Query<&Position, With<Ai>>, // TODO replace with collision system caching positions
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut key_repeat_manager: ResMut<KeyRepeatManager>,
    mut ev_movement: EventWriter<MovementIntent>,
    mut ev_attack: EventWriter<AttackIntent>,
) {
    let Ok((player, player_position)) = q_player.get_single() else {
        return;
    };
    keys.get_just_released()
        .for_each(|key| key_repeat_manager.released(key));

    let mut move_or_attack = |direction: IVec2| {
        let new_position = (player_position.xy.as_ivec2() + direction).as_uvec2();
        if q_monsters
            .iter()
            .any(|position| position.xy == new_position)
        {
            ev_attack.send(AttackIntent {
                actor: player,
                vector: direction,
            });
        } else {
            ev_movement.send(MovementIntent {
                actor: player,
                vector: direction,
            });
        }
        true
    };

    let dt = time.delta_seconds_f64();
    keys.get_pressed()
        .filter(|key| key_repeat_manager.pressed(key, dt))
        .find(|k| match k {
            KeyCode::Up | KeyCode::K => move_or_attack(IVec2::Y),
            KeyCode::Down | KeyCode::J => move_or_attack(IVec2::NEG_Y),
            KeyCode::Left | KeyCode::H => move_or_attack(IVec2::NEG_X),
            KeyCode::Right | KeyCode::L => move_or_attack(IVec2::X),
            KeyCode::Y => move_or_attack(IVec2::new(-1, 1)),
            KeyCode::U => move_or_attack(IVec2::new(1, 1)),
            KeyCode::B => move_or_attack(IVec2::new(-1, -1)),
            KeyCode::N => move_or_attack(IVec2::new(1, -1)),
            _ => false,
        });
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_input.run_if(ai_inactive))
            .insert_resource(KeyRepeatManager::new());
    }
}
