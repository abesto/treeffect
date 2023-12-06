use std::collections::HashMap;

use bevy::prelude::*;

use crate::components::{intents::movement::MovementIntent, player::Player};

const KEY_REPEAT: f64 = 0.250;
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
        held_duration % KEY_REPEAT > (held_duration - dt) % KEY_REPEAT
    }

    fn released(&mut self, key: &KeyCode) {
        self.keys.remove(key);
    }
}

fn handle_input(
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut key_repeat_manager: ResMut<KeyRepeatManager>,
) {
    let Ok(player) = q_player.get_single() else {
        return;
    };
    keys.get_just_released()
        .for_each(|key| key_repeat_manager.released(key));

    let dt = time.delta_seconds_f64();
    let intent = keys
        .get_pressed()
        .filter(|key| key_repeat_manager.pressed(key, dt))
        .find_map(|k| match k {
            KeyCode::Up | KeyCode::K => Some(MovementIntent(IVec2::Y)),
            KeyCode::Down | KeyCode::J => Some(MovementIntent(IVec2::NEG_Y)),
            KeyCode::Left | KeyCode::H => Some(MovementIntent(IVec2::NEG_X)),
            KeyCode::Right | KeyCode::L => Some(MovementIntent(IVec2::X)),
            KeyCode::Y => Some(MovementIntent(IVec2::new(-1, 1))),
            KeyCode::U => Some(MovementIntent(IVec2::new(1, 1))),
            KeyCode::B => Some(MovementIntent(IVec2::new(-1, -1))),
            KeyCode::N => Some(MovementIntent(IVec2::new(1, -1))),
            _ => None,
        });

    if let Some(intent) = intent {
        commands.entity(player).insert(intent);
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_input)
            .insert_resource(KeyRepeatManager::new());
    }
}
