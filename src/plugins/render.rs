use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;
use strum::IntoEnumIterator;

use crate::{
    components::{
        position::Position,
        renderable::{RenderLayer, Renderable},
    },
    plugins::map::resources::{Map, TileType},
};

fn clear(mut q_terminal: Query<&mut Terminal>) {
    let mut terminal = q_terminal.single_mut();
    terminal.clear();
}

fn render_map(mut q_terminal: Query<&mut Terminal>, map: Res<Map>) {
    let mut terminal = q_terminal.single_mut();
    for x in 0..map.size.x {
        for y in 0..map.size.y {
            let xy = UVec2::new(x, y);
            let glyph = match map[&xy] {
                TileType::Wall => '#',
                TileType::Floor => '.',
                TileType::DownStairs => '>',
            };
            terminal.put_char(xy, glyph);
        }
    }
}

fn render_entities(
    mut q_terminal: Query<&mut Terminal>,
    q_entity: Query<(&Position, &Renderable)>,
) {
    let mut terminal = q_terminal.single_mut();
    for render_layer in RenderLayer::iter() {
        for (pos, renderable) in q_entity.iter() {
            if renderable.render_layer == render_layer {
                terminal.put_char(pos.xy, renderable.formatted_tile.clone());
            }
        }
    }
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (clear, render_map, render_entities).chain());
    }
}
