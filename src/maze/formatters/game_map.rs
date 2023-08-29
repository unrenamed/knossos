use crate::{
    maze::{
        formatters::Formatter,
        grid::{Grid, cell::Cell},
    },
    utils::{types::Coords, rand::RandPositions},
};
use std::fmt::Write;

use super::StringWrapper;

pub trait ExtraState {}
pub struct NoStartGoal;
pub struct WithStartGoal {
    start: char,
    goal: char,
}

impl ExtraState for NoStartGoal {}
impl ExtraState for WithStartGoal {}

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
pub struct GameMap<S: ExtraState> {
    state: Box<GameMapState>,
    extra: S,
}

struct GameMapState {
    span: usize,
    wall: char,
    passage: char,
}

impl GameMap<NoStartGoal> {
    /// Returns a new instance of an [GameMap] formatter with a default settings
    pub fn new() -> GameMap<NoStartGoal> {
        GameMap {
            state: Box::new(GameMapState {
                span: 2,
                wall: '#',
                passage: '.',
            }),
            extra: NoStartGoal,
        }
    }

    pub fn with_start_goal(self) -> GameMap<WithStartGoal> {
        GameMap {
            state: self.state,
            extra: WithStartGoal {
                start: 'S',
                goal: 'G',
            },
        }
    }

    /// Sets a span (a distance between two walls) and returns itself
    pub fn span(mut self, span: usize) -> Self {
        self.state.span = span;
        self
    }

    /// Sets a wall and returns itself
    pub fn wall(mut self, wall: char) -> Self {
        self.state.wall = wall;
        self
    }

    /// Sets a passage and returns itself
    pub fn passage(mut self, passage: char) -> Self {
        self.state.passage = passage;
        self
    }
}

impl GameMap<WithStartGoal> {
    /// Sets a goal charachter and returns itself
    pub fn goal(mut self, goal: char) -> Self {
        self.extra.goal = goal;
        self
    }

    /// Sets a start charachter and returns itself
    pub fn start(mut self, start: char) -> Self {
        self.extra.start = start;
        self
    }

    pub fn get_random_start_and_goal_positions(
        &self,
        map: &Vec<char>,
        cols: usize,
        rows: usize,
    ) -> (usize, usize) {
        let mut positions: Vec<Coords> = self
            .iter_possible_start_and_goal_positions(&map, cols, rows)
            .collect();
        
        // shuffle possible positions
        RandPositions::rand(&mut positions);

        let (srow, scol) = positions[0];

        let (grow, gcol) = positions
            .iter()
            .filter(|(nrow, ncol)| *ncol != scol && *nrow != srow)
            .nth(0)
            .unwrap(); // the smallest grid with a single cell formatted into a map has 3 available positions for a goal

        let start_idx = srow * rows + scol;
        let goal_idx = grow * rows + gcol;
        (start_idx, goal_idx)
    }

    fn iter_possible_start_and_goal_positions(
        &self,
        map: &Vec<char>,
        cols: usize,
        rows: usize,
    ) -> impl Iterator<Item = Coords> {
        let mut coords = Vec::new();

        if map.is_empty() {
            return coords.into_iter();
        }

        for row in 0..rows {
            for col in 0..cols {
                if !(row == 0 || row == rows - 1 || col == 0 || col == cols - 1) {
                    continue;
                }

                let adjacent_passages_count = iter_neighbors((row, col), cols, rows)
                    .filter(move |(ny, nx)| map[ny * rows + nx] == self.state.passage)
                    .count();

                if adjacent_passages_count == 0 {
                    continue;
                }

                coords.push((row, col));
            }
        }

        coords.into_iter()
    }
}

impl Default for GameMap<NoStartGoal> {
    fn default() -> Self {
        Self::new()
    }
}

/// An implementation of a formatter
impl Formatter<StringWrapper> for GameMap<NoStartGoal> {
    /// Converts a given grid into the map characters and returns an [StringWrapper] over that image
    fn format(&self, grid: &Grid) -> StringWrapper {
        let mut map = vec![];

        // Span (width of a passage) + 1 (place for a wall)
        let span = self.state.span + 1;

        let map_rows = grid.height() * span + 1;
        let map_cols = grid.width() * span + 1;

        // Add the north wall
        for _ in 0..map_cols {
            map.push(self.state.wall);
        }

        for y in 0..map_rows - 1 {
            // Add the west wall
            map.push(self.state.wall);

            for x in 0..map_cols - 1 {
                // X coordinate of a cell in the grid
                let cx = (x as f64 / span as f64).floor() as usize;
                // Y coordinate of a cell in the grid
                let cy = (y as f64 / span as f64).floor() as usize;

                // Indicates if a row is a last row of a grid cell
                let is_last_row = (y as f64 + 1.0) / span as f64 == cy as f64 + 1.0;
                // Indicates if a column is a last column of a grid cell
                let is_last_col = (x as f64 + 1.0) / span as f64 == cx as f64 + 1.0;

                match (is_last_row, is_last_col) {
                    (false, false) => map.push(self.state.passage),
                    (false, true) => {
                        if grid.is_carved((cx, cy), Cell::EAST) {
                            map.push(self.state.passage);
                        } else {
                            map.push(self.state.wall);
                        }
                    }
                    (true, false) => {
                        if grid.is_carved((cx, cy), Cell::SOUTH) {
                            map.push(self.state.passage);
                        } else {
                            map.push(self.state.wall);
                        }
                    }
                    (true, true) => {
                        if grid.is_carved((cx, cy), Cell::EAST)
                            && grid.is_carved((cx, cy), Cell::SOUTH)
                            && bottom_right_neighbour_exists(cx, cy, grid)
                        {
                            map.push(self.state.passage);
                        } else {
                            map.push(self.state.wall);
                        }
                    }
                }
            }
        }

        // Write map to string
        let string_map = write_map(&map, map_cols);

        StringWrapper(string_map)
    }
}

/// An implementation of a formatter
impl Formatter<StringWrapper> for GameMap<WithStartGoal> {
    /// Converts a given grid into the map characters and returns an [StringWrapper] over that image
    fn format(&self, grid: &Grid) -> StringWrapper {
        let mut map = vec![];

        // Span (width of a passage) + 1 (place for a wall)
        let span = self.state.span + 1;

        let map_rows = grid.height() * span + 1;
        let map_cols = grid.width() * span + 1;

        // Add the north wall
        for _ in 0..map_cols {
            map.push(self.state.wall);
        }

        for y in 0..map_rows - 1 {
            // Add the west wall
            map.push(self.state.wall);

            for x in 0..map_cols - 1 {
                // X coordinate of a cell in the grid
                let cx = (x as f64 / span as f64).floor() as usize;
                // Y coordinate of a cell in the grid
                let cy = (y as f64 / span as f64).floor() as usize;

                // Indicates if a row is a last row of a grid cell
                let is_last_row = (y as f64 + 1.0) / span as f64 == cy as f64 + 1.0;
                // Indicates if a column is a last column of a grid cell
                let is_last_col = (x as f64 + 1.0) / span as f64 == cx as f64 + 1.0;

                match (is_last_row, is_last_col) {
                    (false, false) => map.push(self.state.passage),
                    (false, true) => {
                        if grid.is_carved((cx, cy), Cell::EAST) {
                            map.push(self.state.passage);
                        } else {
                            map.push(self.state.wall);
                        }
                    }
                    (true, false) => {
                        if grid.is_carved((cx, cy), Cell::SOUTH) {
                            map.push(self.state.passage);
                        } else {
                            map.push(self.state.wall);
                        }
                    }
                    (true, true) => {
                        if grid.is_carved((cx, cy), Cell::EAST)
                            && grid.is_carved((cx, cy), Cell::SOUTH)
                            && bottom_right_neighbour_exists(cx, cy, grid)
                        {
                            map.push(self.state.passage);
                        } else {
                            map.push(self.state.wall);
                        }
                    }
                }
            }
        }

        // Get random start and goal points
        let (start_idx, goal_idx) =
            self.get_random_start_and_goal_positions(&map, map_cols, map_rows);
        map[start_idx] = self.extra.start;
        map[goal_idx] = self.extra.goal;

        // Write map to string
        let string_map = write_map(&map, map_cols);

        StringWrapper(string_map)
    }
}

fn bottom_right_neighbour_exists(cx: usize, cy: usize, grid: &Grid) -> bool {
    if cy + 1 >= grid.width() || cx + 1 >= grid.height() {
        return false;
    }

    grid.is_carved((cx + 1, cy + 1), Cell::WEST) && grid.is_carved((cx + 1, cy + 1), Cell::NORTH)
}

fn write_map(map: &Vec<char>, cols: usize) -> String {
    let mut ascii_map: String = String::new();
    for i in 0..map.len() {
        write!(ascii_map, "{}", map[i]).unwrap();
        if (i + 1) % cols == 0 {
            writeln!(ascii_map).unwrap();
        }
    }
    ascii_map
}

fn iter_neighbors((row, col): Coords, cols: usize, rows: usize) -> impl Iterator<Item = Coords> {
    let mut adjacent_coords = Vec::new();

    if row > 0 {
        adjacent_coords.push((row - 1, col));
    }
    if row < rows - 1 {
        adjacent_coords.push((row + 1, col));
    }
    if col > 0 {
        adjacent_coords.push((row, col - 1));
    }
    if col < cols - 1 {
        adjacent_coords.push((row, col + 1));
    }

    adjacent_coords.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_call() {
        let formatter = GameMap::new();
        assert_eq!(2, formatter.state.span);
        assert_eq!('#', formatter.state.wall);
        assert_eq!('.', formatter.state.passage);
    }

    #[test]
    fn default_call() {
        let formatter = GameMap::default();
        assert_eq!(2, formatter.state.span);
        assert_eq!('#', formatter.state.wall);
        assert_eq!('.', formatter.state.passage);
    }

    #[test]
    fn span_change() {
        let formatter = GameMap::new().span(10);
        assert_eq!(10, formatter.state.span);
    }

    #[test]
    fn wall_change() {
        let formatter = GameMap::new().wall('#');
        assert_eq!('#', formatter.state.wall);
    }

    #[test]
    fn passage_change() {
        let formatter = GameMap::new().passage('.');
        assert_eq!('.', formatter.state.passage);
    }

    #[test]
    fn goal_change() {
        let formatter = GameMap::new().with_start_goal().goal('2');
        assert_eq!('2', formatter.extra.goal);
    }

    #[test]
    fn start_change() {
        let formatter = GameMap::new().with_start_goal().start('1');
        assert_eq!('1', formatter.extra.start);
    }

    #[test]
    fn possible_start_and_goal_positions() {
        let formatter = GameMap::new().with_start_goal();
        let map = vec![
            '#', '#', '#', '#', '#', '#', '#', '#', '#', 
            '#', '.', '.', '.', '#', '.', '.', '.', '#', 
            '#', '#', '#', '.', '#', '.', '#', '.', '#',
            '#', '.', '.', '.', '#', '.', '#', '.', '#', 
            '#', '.', '#', '#', '#', '.', '#', '.', '#', 
            '#', '.', '.', '.', '.', '.', '#', '.', '#', 
            '#', '#', '#', '#', '#', '#', '#', '.', '#', 
            '#', '.', '.', '.', '.', '.', '.', '.', '#', 
            '#', '#', '#', '#', '#', '#', '#', '#', '#',
        ];
        let positions = vec![
            (0, 1), (0, 2), (0, 3), (0, 5), (0, 6),
            (0, 7),(1, 0),(1, 8),(2, 8),(3, 0),(3, 8),(4, 0),(4, 8),(5, 0),(5, 8),(6, 8),(7, 0),(7, 8),(8, 1),(8, 2),(8, 3),(8, 4),(8, 5),(8, 6),(8, 7),
        ];
        let result = formatter.iter_possible_start_and_goal_positions(&map, 9, 9);
        assert_eq!(positions, result.collect::<Vec<_>>());
    }

    #[test]
    fn possible_start_and_goal_positions_when_map_is_empty() {
        let formatter = GameMap::new().with_start_goal();
        let map = vec![ ];
        let positions: Vec<Coords> = vec![];
        let result = formatter.iter_possible_start_and_goal_positions(&map, 0, 0);
        assert_eq!(positions, result.collect::<Vec<Coords>>());
    }

    #[test]
    fn format_with_no_start_and_goal() {
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

        let formatter = GameMap::new().span(1);
        let mut grid = generate_maze();
        let actual = formatter.format(&mut grid).0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn format_with_start_and_goal() {
        let mut expected = String::new();
        expected.push_str("#S#######\n");
        expected.push_str("G.#.....#\n");
        expected.push_str("#.#####.#\n");
        expected.push_str("#.....#.#\n");
        expected.push_str("###.###.#\n");
        expected.push_str("#.......#\n");
        expected.push_str("#.#######\n");
        expected.push_str("#.......#\n");
        expected.push_str("#########\n");

        let formatter = GameMap::new().span(1).with_start_goal();
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
