use bevy::math::{IVec2, UVec2};

pub trait UVec2Ext {
    fn as_index(&self, width: u32) -> usize;
    fn offset(&self, by: &IVec2) -> UVec2;
}

impl UVec2Ext for UVec2 {
    fn as_index(&self, width: u32) -> usize {
        (self.x + self.y.saturating_mul(width)) as usize
    }

    fn offset(&self, by: &IVec2) -> UVec2 {
        [
            self.x.saturating_add_signed(by.x),
            self.y.saturating_add_signed(by.y),
        ]
        .into()
    }
}
