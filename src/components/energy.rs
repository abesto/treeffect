use bevy::prelude::*;

#[derive(Component)]
pub struct Energy {
    pub amount: u16,
}

impl Energy {
    pub fn new(amount: u16) -> Self {
        Energy { amount }
    }
}

#[derive(Component)]
pub struct Active;
