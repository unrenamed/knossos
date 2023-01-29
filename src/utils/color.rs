#[derive(Debug, PartialEq, Copy, Clone)]
/// An enumeration over supported color types for filling a maze image with colors
///
/// # Usage
///
/// ```
/// use knossos::{maze::*, Color};
///
/// let formatter = Image::new().background(Color::RGB(0, 0, 0));
/// ```
pub enum Color {
    /// An RGB image
    RGB(u8, u8, u8),
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::RGB(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
        }
    }
}
