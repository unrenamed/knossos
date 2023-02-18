use crate::maze::{
    formatters::Formatter,
    grid::{Grid, cell::Cell},
};
use std::fmt::Write;

use super::StringWrapper;

/// A GameMap formatter for a generated maze
///
/// This can be used for generating a game map for pseudo 3D games that use ray casting algorithm
/// for modeling and rendering the map.
///
/// # Example:
///
/// The span value is 2.
/// ```no_test
/// #############
/// #........#..#
/// #........#..#
/// #..#  ####..#
/// #..#........#
/// #..#........#
/// #..####..#..#
/// #.....#..#..#
/// #.....#..#..#
/// ####..#..#..#
/// #...........#
/// #...........#
/// #############
/// ```
pub struct GameMap {
    span: usize,
    wall: char,
    passage: char,
}

impl GameMap {
    /// Returns a new instance of an [GameMap] formatter with a default settings
    pub fn new() -> GameMap {
        GameMap {
            span: 2,
            wall: '#',
            passage: '.',
        }
    }

    /// Sets a span (a distance between two walls) and returns itself
    pub fn span(mut self, span: usize) -> Self {
        self.span = span;
        self
    }

    /// Sets a wall and returns itself
    pub fn wall(mut self, wall: char) -> Self {
        self.wall = wall;
        self
    }

    /// Sets a passage and returns itself
    pub fn passage(mut self, passage: char) -> Self {
        self.passage = passage;
        self
    }
}

/// An implementation of a formatter
impl Formatter<StringWrapper> for GameMap {
    /// Converts a given grid into the map characters and returns an [StringWrapper] over that image
    fn format(&self, grid: &Grid) -> StringWrapper {
        let mut map = String::new();

        // Span (width of a passage) + 1 (place for a wall)
        let span = self.span + 1;

        add_horizontal_border(&mut map, grid.width(), span, self.wall);

        for y in 0..grid.height() * span {
            add_wall(&mut map, self.wall);

            for x in 0..grid.width() * span {
                // X coordinate of a cell in the grid
                let cx = (x as f64 / span as f64).floor() as usize;
                // Y coordinate of a cell in the grid
                let cy = (y as f64 / span as f64).floor() as usize;

                // Indicates if a row is a last row of a grid cell
                let is_last_row = (y as f64 + 1.0) / span as f64 == cy as f64 + 1.0;
                // Indicates if a column is a last column of a grid cell
                let is_last_col = (x as f64 + 1.0) / span as f64 == cx as f64 + 1.0;

                match (is_last_row, is_last_col) {
                    (false, false) => add_passage(&mut map, self.passage),
                    (false, true) => {
                        if grid.is_carved((cx, cy), Cell::EAST) {
                            add_passage(&mut map, self.passage);
                        } else {
                            add_wall(&mut map, self.wall);
                        }
                    }
                    (true, false) => {
                        if grid.is_carved((cx, cy), Cell::SOUTH) {
                            add_passage(&mut map, self.passage);
                        } else {
                            add_wall(&mut map, self.wall);
                        }
                    }
                    (true, true) => {
                        if grid.is_carved((cx, cy), Cell::EAST)
                            && grid.is_carved((cx, cy), Cell::SOUTH)
                            && bottom_right_neighbour_exists(cx, cy, grid)
                        {
                            add_passage(&mut map, self.passage);
                        } else {
                            add_wall(&mut map, self.wall);
                        }
                    }
                }
            }

            move_to_next_line(&mut map);
        }

        StringWrapper(map)
    }
}

fn add_horizontal_border(map: &mut String, width: usize, span: usize, ch: char) {
    let mut horizontal_border = String::new();
    let border_len = width * span + 1;
    for _ in 0..border_len {
        horizontal_border.push(ch);
    }

    writeln!(map, "{}", horizontal_border).unwrap();
}

fn add_passage(map: &mut String, ch: char) {
    write!(map, "{}", ch).unwrap();
}

fn add_wall(map: &mut String, ch: char) {
    write!(map, "{}", ch).unwrap();
}

fn move_to_next_line(map: &mut String) {
    writeln!(map, "").unwrap();
}

fn bottom_right_neighbour_exists(cx: usize, cy: usize, grid: &Grid) -> bool {
    if cy + 1 >= grid.width() || cx + 1 >= grid.height() {
        return false;
    }

    grid.is_carved((cx + 1, cy + 1), Cell::WEST) && grid.is_carved((cx + 1, cy + 1), Cell::NORTH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_call_default_params() {
        let formatter = GameMap::new();
        assert_eq!(2, formatter.span);
    }

    #[test]
    fn span_change() {
        let formatter = GameMap::new().span(10);
        assert_eq!(10, formatter.span);
    }

    #[test]
    fn wall_change() {
        let formatter = GameMap::new().wall('#');
        assert_eq!('#', formatter.wall);
    }

    #[test]
    fn passage_change() {
        let formatter = GameMap::new().passage('.');
        assert_eq!('.', formatter.passage);
    }

    #[test]
    fn format() {
        let mut expected = String::new();
        expected.push_str("#########\n");
        expected.push_str("#.#.....#\n");
        expected.push_str("#.#####.#\n");
        expected.push_str("#.....#.#\n");
        expected.push_str("###.###.#\n");
        expected.push_str("#.......#\n");
        expected.push_str("#.#######\n");
        expected.push_str("#.......#\n");
        expected.push_str("#########\n");

        let formatter = GameMap {
            span: 1,
            passage: '.',
            wall: '#',
        };
        let mut grid = generate_maze();
        let actual = formatter.format(&mut grid).0;

        assert_eq!(actual, expected);
    }

    fn generate_maze() -> Grid {
        let mut grid = Grid::new(4, 4);

        grid.carve_passage((0, 0), Cell::SOUTH).unwrap();
        grid.carve_passage((0, 1), Cell::EAST).unwrap();
        grid.carve_passage((0, 2), Cell::EAST).unwrap();
        grid.carve_passage((0, 2), Cell::SOUTH).unwrap();
        grid.carve_passage((0, 3), Cell::EAST).unwrap();

        grid.carve_passage((1, 0), Cell::EAST).unwrap();
        grid.carve_passage((1, 1), Cell::EAST).unwrap();
        grid.carve_passage((1, 1), Cell::SOUTH).unwrap();
        grid.carve_passage((1, 2), Cell::EAST).unwrap();
        grid.carve_passage((1, 3), Cell::EAST).unwrap();

        grid.carve_passage((2, 0), Cell::EAST).unwrap();
        grid.carve_passage((2, 2), Cell::EAST).unwrap();
        grid.carve_passage((2, 3), Cell::EAST).unwrap();

        grid.carve_passage((3, 1), Cell::NORTH).unwrap();
        grid.carve_passage((3, 1), Cell::SOUTH).unwrap();

        grid
    }
}
