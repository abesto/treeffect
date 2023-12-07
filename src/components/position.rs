use bevy::{
    ecs::component::Component,
    math::{IVec2, UVec2},
};

#[derive(Component, Clone)]
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

impl From<IVec2> for Position {
    fn from(xy: IVec2) -> Self {
        Position { xy: xy.as_uvec2() }
    }
}
