use bevy::{ecs::bundle::Bundle, render::color::Color};
use bevy_ascii_terminal::TileFormatter;

use crate::{
    behaviors::{actor::ActorBehavior, Behavior},
    components::{
        ai::Ai,
        energy::Energy,
        name::Name,
        position::Position,
        renderable::{RenderLayer, Renderable},
    },
};

type Behaviors = ActorBehavior;

#[derive(Bundle)]
pub struct OrcBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub energy: Energy,
    pub ai: Ai,
    pub name: Name,
    pub behaviors: Behaviors,
}

impl Default for OrcBundle {
    fn default() -> Self {
        OrcBundle {
            renderable: Renderable::new('o'.fg(Color::RED), RenderLayer::Monsters),
            position: Position::default(),
            energy: Energy::new(2),
            ai: Ai::Monster,
            name: Name::new("orc"),
            behaviors: Behaviors::behavior(),
        }
    }
}
