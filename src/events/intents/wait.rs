use bevy::prelude::*;
use bevy_eventlistener::prelude::*;

#[derive(Clone, Event, EntityEvent)]
pub struct WaitIntent {
    #[target]
    pub actor: Entity,
}
