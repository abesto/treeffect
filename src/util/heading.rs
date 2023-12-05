use bevy::math::IVec2;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Hash, Clone, Copy, EnumIter, Debug)]
pub enum Heading {
    North,
    East,
    South,
    West,
}

impl From<Heading> for IVec2 {
    fn from(heading: Heading) -> Self {
        match heading {
            Heading::North => IVec2::Y,
            Heading::East => IVec2::X,
            Heading::South => IVec2::NEG_Y,
            Heading::West => IVec2::NEG_X,
        }
    }
}

impl Heading {
    pub fn rotate_cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn rotate_ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}
