use bevy::{
    ecs::component::Component,
    math::{U64Vec2, U8Vec2},
    reflect::Reflect,
};

/// Basic coords type
pub type Coords = (usize, usize);

/// Auxiliary Bevy component to hold Coords
#[derive(Clone, Debug, Reflect, Component)]
pub struct CoordsComponent {
    coord: Coords,
}

impl From<Coords> for CoordsComponent {
    fn from(value: Coords) -> Self {
        Self { coord: value }
    }
}

impl From<CoordsComponent> for Coords {
    fn from(value: CoordsComponent) -> Self {
        (value.coord.0, value.coord.1)
    }
}

impl From<U8Vec2> for CoordsComponent {
    fn from(value: U8Vec2) -> Self {
        Self {
            coord: (value.x as usize, value.y as usize),
        }
    }
}

impl From<U64Vec2> for CoordsComponent {
    fn from(value: U64Vec2) -> Self {
        Self {
            coord: (value.x as usize, value.y as usize),
        }
    }
}
