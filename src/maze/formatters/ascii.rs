use crate::maze::grid::cell::Cell;
use crate::maze::{formatters::Formatter, grid::Grid};
use std::fmt::Write;
use std::iter;

use super::StringWrapper;

/// A default output type of an Ascii formatter
///
/// # Example:
///
/// ```no_test
///  _______
/// |  ___| |
/// |_  |  _|
/// | | |_  |
/// |_______|
/// ```
pub struct Default;
/// An enhanced output of an Ascii formatter using broader passages and "+" for corners.
///
/// # Example:
///
/// ```no_test
/// +---+---+---+---+
/// |               |
/// +---+---+   +   +
/// |           |   |
/// +   +---+   +   +
/// |   |       |   |
/// +   +---+---+   +
/// |   |           |
/// +---+---+---+---+
/// ```
pub struct Enhanced;

/// An Ascii formatter for a generated maze
pub struct Ascii<Type> {
    _type: Type,
}

impl Ascii<Default> {
    pub fn new() -> Self {
        Ascii { _type: Default }
    }
}

impl Ascii<Enhanced> {
    pub fn new() -> Self {
        Ascii { _type: Enhanced }
    }
}

/// An implementation of a default formatter
impl Formatter<StringWrapper> for Ascii<Default> {
    /// Converts a given grid into ascii characters and returns an [StringWrapper] over that image
    fn format(&self, grid: &Grid) -> StringWrapper {
        let mut result = String::new();

        let top_border = iter::repeat("_")
            .take(grid.width() * 2 - 1)
            .collect::<String>();

        writeln!(result, " {} ", top_border).unwrap();

        for y in 0..grid.height() {
            write!(result, "|").unwrap();

            for x in 0..grid.width() {
                if grid.is_carved((x, y), Cell::SOUTH) {
                    write!(result, " ").unwrap();
                } else {
                    write!(result, "_").unwrap();
                }

                if grid.is_carved((x, y), Cell::EAST) {
                    if grid.is_carved((x, y), Cell::SOUTH)
                        || grid.is_carved((x + 1, y), Cell::SOUTH)
                    {
                        write!(result, " ").unwrap();
                    } else {
                        write!(result, "_").unwrap();
                    }
                } else {
                    write!(result, "|").unwrap();
                }
            }

            writeln!(result, "").unwrap();
        }

        StringWrapper(result)
    }
}

/// An implementation of an enhanced formatter
impl Formatter<StringWrapper> for Ascii<Enhanced> {
    /// Converts a given grid into ascii characters and returns an [StringWrapper] over that image
    fn format(&self, grid: &Grid) -> StringWrapper {
        let mut output = format!("+{}\n", "---+".to_string().repeat(grid.width()));

        for y in 0..grid.height() {
            let mut top_line = "|".to_string();
            let mut bottom_line = "+".to_string();

            for x in 0..grid.width() {
                top_line.push_str("   ");
                let east_boundary = if grid.is_carved((x, y), Cell::EAST) {
                    " "
                } else {
                    "|"
                };
                top_line.push_str(east_boundary);

                let south_boundary = if grid.is_carved((x, y), Cell::SOUTH) {
                    "   "
                } else {
                    "---"
                };
                bottom_line.push_str(&south_boundary);
                bottom_line.push('+');

                if x == grid.width() - 1 {
                    output.push_str(&top_line);
                    output.push_str("\n");
                    output.push_str(&bottom_line);
                    output.push_str("\n");
                }
            }
        }

        StringWrapper(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_default() {
        let mut expected = String::new();
        expected.push_str(" _______ \n");
        expected.push_str("| |___  |\n");
        expected.push_str("|_   _| |\n");
        expected.push_str("|  _____|\n");
        expected.push_str("|_______|\n");

        let formatter = Ascii::<Default>::new();
        let mut grid = generate_maze();
        let actual = formatter.format(&mut grid).0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn format_ehanced() {
        let mut expected = String::new();
        expected.push_str("+---+---+---+---+\n");
        expected.push_str("|   |           |\n");
        expected.push_str("+   +---+---+   +\n");
        expected.push_str("|           |   |\n");
        expected.push_str("+---+   +---+   +\n");
        expected.push_str("|               |\n");
        expected.push_str("+   +---+---+---+\n");
        expected.push_str("|               |\n");
        expected.push_str("+---+---+---+---+\n");

        let formatter = Ascii::<Enhanced>::new();
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
