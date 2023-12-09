use bevy::prelude::*;
use bevy_eventlistener::prelude::*;

#[derive(Clone, Event, EntityEvent)]
pub struct MovementIntent {
    #[target]
    pub actor: Entity,
    pub vector: IVec2,
}
