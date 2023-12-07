use bevy::ecs::component::Component;

#[derive(Component, Clone, PartialEq, Eq)]
pub enum Ai {
    Dog,
    Monster,
}
