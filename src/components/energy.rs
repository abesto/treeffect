use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
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
