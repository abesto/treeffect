use bevy::ecs::component::Component;
use bevy_ascii_terminal::FormattedTile;
use strum_macros::EnumIter;

#[derive(EnumIter, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RenderLayer {
    Items,
    Monsters,
    Player,
    Particle,
}

#[derive(Component, Clone)]
pub struct Renderable {
    pub formatted_tile: FormattedTile,
    pub render_layer: RenderLayer,
}

impl Renderable {
    pub fn new(formatted_tile: FormattedTile, render_layer: RenderLayer) -> Self {
        Renderable {
            formatted_tile,
            render_layer,
        }
    }
}
