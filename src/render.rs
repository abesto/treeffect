use bevy::prelude::*;
use bevy_ascii_terminal::{GridPoint, Terminal};

use crate::map::{Map, Tile};

fn clear(mut q_terminal: Query<&mut Terminal>) {
    let mut terminal = q_terminal.single_mut();
    terminal.clear();
}

fn render_map(mut q_terminal: Query<&mut Terminal>, map: Res<Map>) {
    let mut terminal = q_terminal.single_mut();
    for x in 0..map.size.width() {
        for y in 0..map.size.height() {
            let xy = [x, y];
            let glyph = match map[xy] {
                Tile::Wall => '#',
                Tile::Floor => '.',
                Tile::DownStairs => '>',
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
