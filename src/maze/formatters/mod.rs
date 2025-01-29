//! Formatters for converting a generated maze into other data types

mod ascii;
mod game_map;
mod image;

use crate::maze::grid::Grid;
use ::image::RgbImage;
use std::{fs::File, io::Write};

pub use self::image::Image;
use super::errors::MazeSaveError;
pub use ascii::{AsciiNarrow, AsciiBroad};
pub use game_map::GameMap;

/// A trait for maze formatters
pub trait Formatter<T>
where
    T: Saveable,
{
    /// Returns a given grid converted into a given type that implements [Saveable]
    fn format(&self, grid: &Grid) -> T;
}

/// A trait for data wrappers that must be returned after formatting the grid
pub trait Saveable {
    /// Saves a given object into a file
    ///
    /// In case of success, returns the string with a success message.
    /// Otherwise, returns a [MazeSaveError] with a custom reason message.
    fn save(&self, path: &str) -> Result<String, MazeSaveError>;
}

/// A custom wrapper over [RgbImage] for converting a maze to an image
pub struct ImageWrapper(pub RgbImage);

/// An implementation of [Saveable] for saving a maze image into a file
impl Saveable for ImageWrapper {
    /// Saves an image to a file to a given path
    fn save(&self, path: &str) -> Result<String, MazeSaveError> {
        if let Err(reason) = self.0.save(path) {
            return Err(MazeSaveError {
                reason: reason.to_string(),
            });
        }

        Ok(format!("Maze was successfully saved as an image: {}", path))
    }
}

/// A custom wrapper over [std::string::String](std::string::String) for converting a maze into
/// string characters
pub struct StringWrapper(pub String);

/// An implementation of [Saveable] for saving a maze string into a text file
impl Saveable for StringWrapper {
    /// Saves a maze string to a file to a given path
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
            Err(why) => Err(MazeSaveError {
                reason: format!("Couldn't write to {}: {}", path.display(), why),
            }),
            Ok(_) => Ok(format!(
                "Maze was successfully written to a file: {}",
                path.display()
            )),
        }
    }
}
