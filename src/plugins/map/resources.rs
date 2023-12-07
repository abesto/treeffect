use bevy::{
    ecs::system::Resource,
    math::{IVec2, URect, UVec2},
};

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

    /*
    pub fn idx_xy(&self, idx: usize) -> UVec2 {
        let idx_u32: u32 = idx.try_into().unwrap();
        let y = idx_u32 / self.size.x;
        [idx_u32 - y * self.size.x, y].into()
    }
    */

    /*
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }
    */

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

    pub fn astar(&self, from: &UVec2, to: &UVec2) -> Option<(Vec<UVec2>, i32)> {
        let ito = to.as_ivec2();
        pathfinding::directed::astar::astar(
            from,
            |position| {
                let mut successors = Vec::with_capacity(8);
                for direction in [
                    NORTH, SOUTH, EAST, WEST, NORTH_EAST, NORTH_WEST, SOUTH_EAST, SOUTH_WEST,
                ] {
                    let new_position = (position.as_ivec2() + direction).as_uvec2();
                    if self.is_walkable(&new_position) {
                        successors.push((new_position, 1));
                    }
                }
                successors
            },
            |position| position.as_ivec2().distance_squared(ito),
            |position| position == to,
        )
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
