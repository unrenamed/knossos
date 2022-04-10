use std::fmt;

#[derive(Debug, Clone)]
/// A maze save error
///
/// Represents a custom error when a maze cannot be saved into a file
pub struct MazeSaveError {
    /// A reason why a maze cannot be saved into a file
    pub reason: String,
}

/// An implementation of [fmt::Display](fmt::Display) trait
impl fmt::Display for MazeSaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot save maze to file. Reason: {}", self.reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let error = MazeSaveError {
            reason: String::from("It's a fake reason"),
        };

        assert_eq!(
            error.to_string(),
            "Cannot save maze to file. Reason: It's a fake reason"
        )
    }
}
