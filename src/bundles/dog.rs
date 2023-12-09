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
pub struct DogBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub energy: Energy,
    pub ai: Ai,
    pub name: Name,
    pub behaviors: Behaviors,
}

impl Default for DogBundle {
    fn default() -> Self {
        DogBundle {
            renderable: Renderable::new('d'.fg(Color::WHITE), RenderLayer::Monsters),
            energy: Energy::new(1),
            ai: Ai::Dog,
            name: Name::new("Fluffy"),
            behaviors: Behaviors::behavior(),
            position: Position::default(),
        }
    }
}
