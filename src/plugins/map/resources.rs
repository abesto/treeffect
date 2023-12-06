use bevy::{
    ecs::system::Resource,
    math::{IVec2, URect, UVec2},
};
use bracket_pathfinding::prelude::SmallVec;

use crate::util::{ivec2_ext::*, UVec2Ext};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Map {
    pub size: UVec2,
    tiles: Vec<TileType>,
}

impl Resource for Map {}

impl Map {
    pub fn new<T: Into<UVec2>>(size: T) -> Self {
        let size = size.into();
        let tiles = vec![TileType::Wall; (size.x * size.y) as usize];
        Self { size, tiles }
    }

    pub fn xy_idx(&self, xy: &UVec2) -> usize {
        xy.as_index(self.size.x)
    }

    pub fn idx_pos(&self, idx: usize) -> UVec2 {
        let idx_u32: u32 = idx.try_into().unwrap();
        let y = idx_u32 / self.size.x;
        [idx_u32 - y * self.size.x, y].into()
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn contains(&self, xy: &UVec2) -> bool {
        xy.x < self.size.x && xy.y < self.size.y
    }

    pub fn clamp(&self, xy: &UVec2) -> UVec2 {
        xy.min(self.size - UVec2::ONE)
    }

    pub fn iclamp(&self, xy: &IVec2) -> UVec2 {
        if xy.x < 0 || xy.y < 0 {
            return UVec2::ZERO;
        }
        self.clamp(&xy.as_uvec2())
    }

    pub fn get(&self, position: &UVec2) -> Option<TileType> {
        self.tiles.get(self.xy_idx(position)).copied()
    }

    pub fn rect(&self) -> URect {
        URect::from_corners(UVec2::ZERO, self.size - UVec2::ONE)
    }

    pub fn is_walkable(&self, position: &UVec2) -> bool {
        self.get(position) == Some(TileType::Floor)
    }
}

impl std::ops::Index<(u32, u32)> for Map {
    type Output = TileType;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.tiles[self.xy_idx(&index.into())]
    }
}

impl std::ops::IndexMut<(u32, u32)> for Map {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let idx = self.xy_idx(&index.into());
        &mut self.tiles[idx]
    }
}

impl std::ops::Index<&UVec2> for Map {
    type Output = TileType;

    fn index(&self, index: &UVec2) -> &Self::Output {
        &self.tiles[self.xy_idx(index)]
    }
}

impl std::ops::IndexMut<&UVec2> for Map {
    fn index_mut(&mut self, index: &UVec2) -> &mut Self::Output {
        let idx = self.xy_idx(index);
        &mut self.tiles[idx]
    }
}

impl bracket_pathfinding::prelude::BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        idx < self.tiles.len() && self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let position = self.idx_pos(idx).as_ivec2();

        [
            NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
        ]
        .iter()
        .map(|vector| position + *vector)
        .filter(|candidate| self.is_walkable(&candidate.as_uvec2()))
        .map(|exit| {
            (
                self.xy_idx(&exit.as_uvec2()),
                (position.distance_squared(exit) as f32).sqrt(),
            )
        })
        .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = self.idx_pos(idx1).as_ivec2();
        let p2 = self.idx_pos(idx2).as_ivec2();
        (p1.distance_squared(p2) as f32).sqrt()
    }
}
