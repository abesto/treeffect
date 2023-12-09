use bevy::prelude::*;
use bevy_eventlistener::prelude::*;

#[derive(Clone, Event, EntityEvent)]
pub struct AttackIntent {
    // This reads weird, watch out: it's the target of the event, but the attacker in the attack.
    #[target]
    pub actor: Entity,
    pub vector: IVec2,
}
