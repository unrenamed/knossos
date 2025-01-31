use std::fmt;

#[derive(Debug, Clone)]
/// An orthogonal maze builder error
///
/// Represents a custom error when using the orthogonal maze builder
pub struct BuildError {
    /// A reason why a maze cannot be built
    pub reason: String,
}

impl BuildError {
    /// Defines reason why was not able to build maze
    pub fn reason(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

/// An implementation of [fmt::Display](fmt::Display) trait
impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cannot build maze. Reason: Algorithm `{}` doesn't support `start_coords`",
            self.reason
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let error = BuildError {
            reason: String::from("It's a fake reason"),
        };

        assert_eq!(
            error.to_string(),
            "Cannot build maze. Reason: Algorithm `It's a fake reason` doesn't support `start_coords`"
        )
    }
}
