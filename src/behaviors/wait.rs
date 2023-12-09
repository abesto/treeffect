use bevy::prelude::*;
use bevy_eventlistener::prelude::*;

use crate::{
    behavior,
    events::{intents::wait::WaitIntent, took_turn::TookTurn},
};

fn wait(wait: Listener<WaitIntent>, mut ev_took_turn: EventWriter<TookTurn>) {
    ev_took_turn.send(wait.listener().into());
}

behavior!(wait);
