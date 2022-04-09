mod ascii;
mod game_map;
mod image;

use crate::maze::grid::Grid;
use ::image::RgbImage;
use std::{fs::File, io::Write};

pub use self::image::Image;
use super::errors::MazeSaveError;
pub use ascii::Ascii;
pub use game_map::GameMap;

// Public interface for a maze formatters
pub trait Formatter<T>
where
    T: Saveable,
{
    fn format(&self, grid: &Grid) -> T;
}

// Public interface for a data wrappers that must be returned after formatting the grid
pub trait Saveable {
    fn save(&self, path: &str) -> Result<String, MazeSaveError>;
}

// Custom wrapper over image::RgbImage struct that must be returned when converting a maze to an
// image
pub struct ImageWrapper(pub RgbImage);

// Images with a maze are to be saved as image files require this implementation
impl Saveable for ImageWrapper {
    fn save(&self, path: &str) -> Result<String, MazeSaveError> {
        if let Err(reason) = self.0.save(path) {
            return Err(MazeSaveError {
                reason: reason.to_string(),
            });
        }

        Ok(format!("Maze was successfully saved as an image: {}", path))
    }
}

// Custom wrapper over Rust's String struct that must be returned when converting a maze to a string
// characters
pub struct StringWrapper(pub String);

// Text-like mazes are to be saved into text files require this implementation
impl Saveable for StringWrapper {
    fn save(&self, path: &str) -> Result<String, MazeSaveError> {
        let path = match std::env::current_dir() {
            Err(why) => {
                return Err(MazeSaveError {
                    reason: format!("Couldn't find path to current dir: {}", why),
                })
            }
            Ok(dir) => dir.join(path),
        };

        let mut file = match File::create(&path) {
            Err(why) => {
                return Err(MazeSaveError {
                    reason: format!("Couldn't create {}: {}", path.display(), why),
                })
            }
            Ok(file) => file,
        };

        match file.write_all(self.0.as_bytes()) {
            Err(why) => {
                return Err(MazeSaveError {
                    reason: format!("Couldn't write to {}: {}", path.display(), why),
                })
            }
            Ok(_) => Ok(format!(
                "Maze was successfully written to a file: {}",
                path.display()
            )),
        }
    }
}
