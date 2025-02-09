use std::fmt;

use bevy::{
    ecs::{component::Component, system::Resource},
    math::{U64Vec2, U8Vec2},
    reflect::Reflect,
};

/// Basic coords type
pub type Coords = (usize, usize);

/// Auxiliary Bevy component to hold Coords
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Component, Hash)]
pub struct CoordsComponent {
    pub(crate) coord: Coords,
}

impl fmt::Display for CoordsComponent {
    /// Writes a formatted maze into a buffer
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.coord.0, self.coord.1)?;
        Ok(())
    }
}

impl CoordsComponent {
    /// Creates new [`CoordsComponent`]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { coord: (x, y) }
    }

    /// Returns `x` and `y` values for [`CoordsComponent`]
    pub const fn xy(&self) -> (usize, usize) {
        (self.coord.0, self.coord.1)
    }
}

/// Maze cell size
#[derive(Clone, Debug, Reflect, Resource)]
pub struct CellSize(pub f32);

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

/// Auxiliary Bevy component that holds the Start Coords of Pathfinding
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Component)]
pub struct Start;

/// Auxiliary Bevy component that holds the Goal Coords of Pathfinding
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Component)]
pub struct Goal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_coords() {
        let component: CoordsComponent = (42, 42).into();
        let expected = CoordsComponent::new(42, 42);

        assert_eq!(component, expected);

        assert_eq!(component.xy(), (42, 42));
    }

    #[test]
    fn from_coords_component() {
        let component = CoordsComponent { coord: (42, 42) };
        let component: Coords = component.into();

        assert_eq!(component, (42, 42));
    }

    #[test]
    fn from_u8vec() {
        let component: CoordsComponent = U8Vec2::from_array([42, 42]).into();
        let expected = CoordsComponent { coord: (42, 42) };

        assert_eq!(component, expected);
    }

    #[test]
    fn from_u64vec() {
        let component: CoordsComponent = U64Vec2::from_array([42, 42]).into();
        let expected = CoordsComponent { coord: (42, 42) };

        assert_eq!(component, expected);
    }
}
