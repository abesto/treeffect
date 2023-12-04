use bevy::prelude::*;

use super::{Map, Tile};

pub struct MapPlugin {
    pub size: IVec2,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let mut map = Map::new(self.size);
        map[[1, 1]] = Tile::Floor;
        app.insert_resource(map);
    }
}
