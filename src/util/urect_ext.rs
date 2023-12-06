use bevy::prelude::*;

pub trait URectExt {
    fn point_set(&self) -> Vec<UVec2>;

    fn north_west(&self) -> UVec2;
    fn north_east(&self) -> UVec2;
    fn south_west(&self) -> UVec2;
    fn south_east(&self) -> UVec2;

    fn tile_width(&self) -> u32;
    fn tile_height(&self) -> u32;
}

impl URectExt for URect {
    fn point_set(&self) -> Vec<UVec2> {
        let mut points = Vec::new();
        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                points.push(UVec2::new(x, y));
            }
        }
        points
    }

    fn north_west(&self) -> UVec2 {
        UVec2::new(self.min.x, self.max.y)
    }

    fn north_east(&self) -> UVec2 {
        UVec2::new(self.max.x, self.max.y)
    }

    fn south_west(&self) -> UVec2 {
        UVec2::new(self.min.x, self.min.y)
    }

    fn south_east(&self) -> UVec2 {
        UVec2::new(self.max.x, self.min.y)
    }

    fn tile_height(&self) -> u32 {
        self.height() + 1
    }

    fn tile_width(&self) -> u32 {
        self.width() + 1
    }
}

pub fn urect_with_size(x0: u32, y0: u32, width: u32, height: u32) -> URect {
    URect::new(x0, y0, x0 + width - 1, y0 + height - 1)
}
