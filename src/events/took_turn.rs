use bevy::prelude::*;

#[derive(Event)]
pub struct TookTurn {
    pub actor: Entity,
}

impl From<Entity> for TookTurn {
    fn from(actor: Entity) -> Self {
        TookTurn { actor }
    }
}
