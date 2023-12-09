use bevy::prelude::*;
use bevy_eventlistener::prelude::*;

use crate::events::intents::{attack::AttackIntent, wait::WaitIntent};

use self::intents::movement::MovementIntent;

pub mod intents;
pub mod took_turn;

// short for "event listener plugins"
macro_rules! elp {
    ($app:ident, $($events:ident),+ $(,)?) => {
        $($app.add_plugins(EventListenerPlugin::<$events>::default());)+
    };
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<took_turn::TookTurn>();
        elp!(app, MovementIntent, AttackIntent, WaitIntent);
    }
}
