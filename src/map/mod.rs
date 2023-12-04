use bevy::{ecs::system::Resource, math::IVec2};
use bevy_ascii_terminal::GridPoint;

mod plugin;
pub use plugin::MapPlugin;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Tile {
    Wall,
    Floor,
    DownStairs,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Map {
    pub size: IVec2,
    tiles: Vec<Tile>,
}

impl Resource for Map {}

impl Map {
    fn new(size: IVec2) -> Self {
        let tiles = vec![Tile::Wall; (size.width() * size.height()) as usize];
        Self { size, tiles }
    }

    pub fn xy_idx(&self, xy: impl GridPoint) -> usize {
        xy.as_index(self.size.x() as usize)
    }

    pub fn idx_pos(&self, idx: usize) -> [i32; 2] {
        let idx_i32: i32 = idx.try_into().unwrap();
        let y = idx_i32 / self.size.width();
        [idx_i32 - y * self.size.width(), y]
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn contains(&self, xy: impl GridPoint) -> bool {
        xy.x() >= 0 && xy.y() >= 0 && xy.x() < self.size.width() && xy.y() < self.size.height()
    }

    pub fn clamp(&self, xy: impl GridPoint) -> IVec2 {
        xy.as_ivec2().min(self.size - IVec2::new(1, 1))
    }
}

impl<T> std::ops::Index<T> for Map
where
    T: GridPoint,
{
    type Output = Tile;

    fn index(&self, index: T) -> &Self::Output {
        &self.tiles[self.xy_idx(index)]
    }
}

impl<T> std::ops::IndexMut<T> for Map
where
    T: GridPoint,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let idx = self.xy_idx(index);
        &mut self.tiles[idx]
    }
}
