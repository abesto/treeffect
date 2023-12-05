use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

use crate::map::{Map, TileType};

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

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (clear, render_map).chain());
    }
}
