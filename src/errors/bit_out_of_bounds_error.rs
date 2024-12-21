use std::fmt;

#[derive(Debug)]
pub struct BitOutOfBoundsError;

impl fmt::Display for BitOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bit index is out of bounds")
    }
}

impl std::error::Error for BitOutOfBoundsError {}