use bevy::{ecs::component::Component, math::UVec2};

#[derive(Component)]
pub struct Position {
    pub xy: UVec2,
}

impl Default for Position {
    fn default() -> Self {
        Position { xy: UVec2::ZERO }
    }
}

impl From<UVec2> for Position {
    fn from(xy: UVec2) -> Self {
        Position { xy }
    }
}
