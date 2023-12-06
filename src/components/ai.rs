use bevy::ecs::component::Component;

#[derive(Component, PartialEq, Eq)]
pub enum Ai {
    Dog,
}
