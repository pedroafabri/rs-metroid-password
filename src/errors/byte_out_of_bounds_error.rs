use std::fmt;

#[derive(Debug)]
pub struct ByteOutOfBoundsError;

impl fmt::Display for ByteOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Byte index is out of bounds")
    }
}

impl std::error::Error for ByteOutOfBoundsError {}