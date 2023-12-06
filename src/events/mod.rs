use bevy::prelude::*;

pub mod took_turn;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<took_turn::TookTurn>();
    }
}
