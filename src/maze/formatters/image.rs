use crate::maze::grid::cell::Cell;
use crate::maze::{formatters::Formatter, grid::Grid};
use crate::utils::color::Color;
use crate::utils::types::Coords;
use image::{ImageBuffer, RgbImage};

use super::ImageWrapper;

/// An Image formatter for a generated maze
pub struct Image {
    wall_width: usize,
    passage_width: usize,
    margin: usize,
    background_color: Color,
    foreground_color: Color,
}

impl Image {
    /// Returns a new instance of an [Image] formatter with a default settings
    pub const fn new() -> Image {
        Image {
            wall_width: 40,
            passage_width: 40,
            background_color: Color::RGB(250, 250, 250),
            foreground_color: Color::RGB(0, 0, 0),
            margin: 50,
        }
    }

    /// Sets a wall width and returns itself
    pub const fn wall(mut self, width: usize) -> Self {
        self.wall_width = width;
        self
    }

    /// Sets a passage width and returns itself
    pub const fn passage(mut self, width: usize) -> Self {
        self.passage_width = width;
        self
    }

    /// Sets a background color and returns itself
    pub const fn background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Sets a maze (foreground) color and returns itself
    pub const fn foreground(mut self, color: Color) -> Self {
        self.foreground_color = color;
        self
    }

    /// Sets a margin (a distance between a maze and the image borders) and returns itself
    pub const fn margin(mut self, value: usize) -> Self {
        self.margin = value;
        self
    }

    const fn cell_width(&self) -> usize {
        self.wall_width * 2 + self.passage_width
    }

    const fn sizes(&self, grid: &Grid) -> (usize, usize) {
        // To calculate maze's width and height we use a simple formula that multiplies a single
        // cell width and a number of cells (in a row or column). However, since two cells
        // have a single joint wall, we do the subtraction of the joint walls from the
        // preceding width
        let maze_width = self.cell_width() * grid.width() - (grid.width() - 1) * self.wall_width;
        let maze_height = self.cell_width() * grid.height() - (grid.height() - 1) * self.wall_width;

        let image_width = maze_width + self.margin * 2;
        let image_height = maze_height + self.margin * 2;

        (image_width, image_height)
    }

    fn fill_background(&self, image: &mut RgbImage) {
        for (_, _, pixel) in image.enumerate_pixels_mut() {
            *pixel = match self.background_color {
                Color::RGB(r, g, b) => image::Rgb([r, g, b]),
            }
        }
    }

    fn draw_maze(&self, image: &mut RgbImage, grid: &Grid) {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                self.draw_cell((x, y), grid, image);
            }
        }
    }

    fn draw_cell(&self, coords: Coords, grid: &Grid, image: &mut RgbImage) {
        let (x, y) = coords;
        let inner_cell_width = self.cell_width() - self.wall_width;
        let cell_x = x * inner_cell_width + self.margin;
        let cell_y = y * inner_cell_width + self.margin;

        for py in cell_y..cell_y + self.cell_width() {
            for px in cell_x..cell_x + self.cell_width() {
                // A cell consists of two main zones: its walls and some empty space between them
                // called "a passage". To draw a cell, the following code checks some particular
                // zones and skips filling pixels with color in case a wall should not display or
                // it's a cell passage. In all other cases, we fill pixels with a given color

                // Top left corner must display only if either Northern or Western wall exists
                if px >= cell_x
                    && px <= cell_x + self.wall_width
                    && py >= cell_y
                    && py <= cell_y + self.wall_width
                    && grid.is_carved(coords, Cell::NORTH)
                    && grid.is_carved(coords, Cell::WEST)
                {
                    continue;
                }

                // Northern wall must display only if there is no passage carved to North
                if px >= cell_x + self.wall_width
                    && px <= cell_x + inner_cell_width
                    && py >= cell_y
                    && py <= cell_y + self.wall_width
                    && grid.is_carved(coords, Cell::NORTH)
                {
                    continue;
                }

                // Top right corner must display only if either Northern or Eastern wall exists
                if px >= cell_x + inner_cell_width
                    && px <= cell_x + self.cell_width()
                    && py >= cell_y
                    && py <= cell_y + self.wall_width
                    && grid.is_carved(coords, Cell::NORTH)
                    && grid.is_carved(coords, Cell::EAST)
                {
                    continue;
                }

                // Western wall must display only if there is no passage carved to West
                if px >= cell_x
                    && px <= cell_x + self.wall_width
                    && py >= cell_y + self.wall_width
                    && py <= cell_y + inner_cell_width
                    && grid.is_carved(coords, Cell::WEST)
                {
                    continue;
                }

                // Cell's passage must not be colored, i.e. it remains same as an image background
                if px >= cell_x + self.wall_width
                    && px <= cell_x + inner_cell_width
                    && py >= cell_y + self.wall_width
                    && py <= cell_y + inner_cell_width
                {
                    continue;
                }

                // Eastern wall must display only if there is no passage carved to East
                if px >= cell_x + inner_cell_width
                    && px <= cell_x + self.cell_width()
                    && py >= cell_y + self.wall_width
                    && py <= cell_y + inner_cell_width
                    && grid.is_carved(coords, Cell::EAST)
                {
                    continue;
                }

                // Bottom left corner must display only if either Southern or Western wall exists
                if px >= cell_x
                    && px <= cell_x + self.wall_width
                    && py >= cell_y + inner_cell_width
                    && py <= cell_y + self.cell_width()
                    && grid.is_carved(coords, Cell::SOUTH)
                    && grid.is_carved(coords, Cell::WEST)
                {
                    continue;
                }

                // Southern wall must display only if there is no passage carved to South
                if px >= cell_x + self.wall_width
                    && px <= cell_x + inner_cell_width
                    && py >= cell_y + inner_cell_width
                    && py <= cell_y + self.cell_width()
                    && grid.is_carved(coords, Cell::SOUTH)
                {
                    continue;
                }

                // Bottom right corner must display only if either Southern or Eastern wall exists
                if px >= cell_x + inner_cell_width
                    && px <= cell_x + self.cell_width()
                    && py >= cell_y + inner_cell_width
                    && py <= cell_y + self.cell_width()
                    && grid.is_carved(coords, Cell::SOUTH)
                    && grid.is_carved(coords, Cell::EAST)
                {
                    continue;
                }

                // Fill the remaining pixels with a given color
                *image.get_pixel_mut(px as u32, py as u32) = match self.foreground_color {
                    Color::RGB(r, g, b) => image::Rgb([r, g, b]),
                }
            }
        }

        let is_rightmost_cell = x == grid.width() - 1;
        let is_bottommost_cell = y == grid.height() - 1;

        // Ensure the rightmost and bottommost walls are fully drawn to prevent missing edges
        // at the outer boundary of the grid. This guarantees a closed maze structure.
        if is_rightmost_cell || is_bottommost_cell {
            let cell_right_x = cell_x + self.cell_width() - 1;
            let cell_bottom_y = cell_y + self.cell_width() - 1;

            for py in cell_y..=cell_bottom_y {
                for px in cell_x..=cell_right_x {
                    if (is_rightmost_cell && px > cell_right_x - self.wall_width)
                        || (is_bottommost_cell && py > cell_bottom_y - self.wall_width)
                    {
                        *image.get_pixel_mut(px as u32, py as u32) = match self.foreground_color {
                            Color::RGB(r, g, b) => image::Rgb([r, g, b]),
                        }
                    }
                }
            }
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

/// An implementation of a formatter
impl Formatter<ImageWrapper> for Image {
    /// Converts a given grid into an image and returns an [ImageWrapper] over that image
    fn format(&self, grid: &Grid) -> ImageWrapper {
        let (width, height) = self.sizes(grid);
        let mut image: RgbImage = ImageBuffer::new(width as u32, height as u32);

        self.fill_background(&mut image);
        self.draw_maze(&mut image, grid);

        ImageWrapper(image)
    }
}

#[cfg(test)]
mod tests {
    use image::EncodableLayout;

    use crate::maze::grid::cell::Cell;

    use super::*;

    #[test]
    fn new_call_default_params() {
        let image = Image::new();
        assert_eq!(40, image.wall_width);
        assert_eq!(40, image.passage_width);
        assert_eq!(Color::RGB(250, 250, 250), image.background_color);
        assert_eq!(Color::RGB(0, 0, 0), image.foreground_color);
        assert_eq!(50, image.margin);
    }

    #[test]
    fn params_change() {
        let image = Image::new()
            .wall(10)
            .passage(5)
            .background(Color::RGB(1, 1, 1))
            .foreground(Color::RGB(100, 100, 100))
            .margin(20);

        assert_eq!(10, image.wall_width);
        assert_eq!(5, image.passage_width);
        assert_eq!(Color::RGB(1, 1, 1), image.background_color);
        assert_eq!(Color::RGB(100, 100, 100), image.foreground_color);
        assert_eq!(20, image.margin);
    }

    #[test]
    fn format_zero_margin() {
        let formatter = Image::new().wall(1).passage(1).margin(0);
        let grid = generate_maze();

        let actual = formatter.format(&grid).into_inner();
        let expected = image::open("tests/fixtures/zero_margin_maze.png").unwrap();
        assert_eq!(actual.as_bytes(), expected.as_bytes());
    }

    #[test]
    fn format_nonzero_margin() {
        let formatter = Image::new().wall(1).passage(1).margin(1);
        let grid = generate_maze();

        let actual = formatter.format(&grid).into_inner();
        let expected = image::open("tests/fixtures/nonzero_margin_maze.png").unwrap();

        assert_eq!(actual.as_bytes(), expected.as_bytes());
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
