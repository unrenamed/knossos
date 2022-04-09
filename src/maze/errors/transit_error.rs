use crate::utils::types::Coords;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TransitError {
    pub coords: Coords,
    pub reason: String,
}

impl fmt::Display for TransitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y) = self.coords;

        write!(
            f,
            "Cannot move to a cell. Reason: {}. Coords: x = {}, y = {}",
            self.reason, x, y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let error = TransitError {
            coords: (0, 0),
            reason: String::from("It's a fake reason"),
        };

        assert_eq!(
            error.to_string(),
            "Cannot move to a cell. Reason: It's a fake reason. Coords: x = 0, y = 0"
        )
    }
}
