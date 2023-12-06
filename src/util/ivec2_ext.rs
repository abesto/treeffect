use bevy::math::IVec2;

pub const NORTH: IVec2 = IVec2::new(0, 1);
pub const SOUTH: IVec2 = IVec2::new(0, -1);
pub const EAST: IVec2 = IVec2::new(1, 0);
pub const WEST: IVec2 = IVec2::new(-1, 0);
pub const NORTH_EAST: IVec2 = IVec2::new(1, 1);
pub const NORTH_WEST: IVec2 = IVec2::new(-1, 1);
pub const SOUTH_EAST: IVec2 = IVec2::new(1, -1);
pub const SOUTH_WEST: IVec2 = IVec2::new(-1, -1);

pub trait IVec2Ext {
    fn perp_cw(&self) -> IVec2;
}

impl IVec2Ext for IVec2 {
    fn perp_cw(&self) -> IVec2 {
        IVec2::new(self.y, -self.x)
    }
}
