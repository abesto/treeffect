use bevy::prelude::*;

mod attack;
mod movement;
mod wait;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            movement::MovementPlugin,
            wait::WaitPlugin,
            attack::AttackPlugin,
        ));
    }
}
