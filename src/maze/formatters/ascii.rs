use crate::maze::grid::pole::Pole;
use crate::maze::{formatters::Formatter, grid::Grid};
use std::fmt::Write;
use std::iter;

use super::StringWrapper;

pub struct Ascii;

impl Formatter<StringWrapper> for Ascii {
    fn format(&self, grid: &Grid) -> StringWrapper {
        let mut result = String::new();

        let top_border = iter::repeat("_")
            .take(grid.width() * 2 - 1)
            .collect::<String>();

        writeln!(result, " {} ", top_border).unwrap();

        for y in 0..grid.height() {
            write!(result, "|").unwrap();

            for x in 0..grid.width() {
                let walls = grid.get_cell((x, y)).get_walls();

                if walls.carved(Pole::S) {
                    write!(result, " ").unwrap();
                } else {
                    write!(result, "_").unwrap();
                }

                if walls.carved(Pole::E) {
                    let next_cell_walls = grid.get_cell((x + 1, y)).get_walls();
                    if walls.carved(Pole::S) || next_cell_walls.carved(Pole::S) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        let mut expected = String::new();
        expected.push_str(" _______ \n");
        expected.push_str("| |___  |\n");
        expected.push_str("|_   _| |\n");
        expected.push_str("|  _____|\n");
        expected.push_str("|_______|\n");

        let formatter = Ascii;
        let mut grid = generate_maze();
        let actual = formatter.format(&mut grid).0;

        assert_eq!(actual, expected);
    }

    fn generate_maze() -> Grid {
        let mut grid = Grid::new(4, 4);

        grid.carve_passage((0, 0), Pole::S).unwrap();
        grid.carve_passage((0, 1), Pole::E).unwrap();
        grid.carve_passage((0, 2), Pole::E).unwrap();
        grid.carve_passage((0, 2), Pole::S).unwrap();
        grid.carve_passage((0, 3), Pole::E).unwrap();

        grid.carve_passage((1, 0), Pole::E).unwrap();
        grid.carve_passage((1, 1), Pole::E).unwrap();
        grid.carve_passage((1, 1), Pole::S).unwrap();
        grid.carve_passage((1, 2), Pole::E).unwrap();
        grid.carve_passage((1, 3), Pole::E).unwrap();

        grid.carve_passage((2, 0), Pole::E).unwrap();
        grid.carve_passage((2, 2), Pole::E).unwrap();
        grid.carve_passage((2, 3), Pole::E).unwrap();

        grid.carve_passage((3, 1), Pole::N).unwrap();
        grid.carve_passage((3, 1), Pole::S).unwrap();

        grid
    }
}
