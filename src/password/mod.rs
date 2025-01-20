use crate::errors::{BitOutOfBoundsError, ByteOutOfBoundsError};

pub struct Password {
    high: u128,
    low: u16
}

impl Password {
    pub fn new() -> Password {
        Password {
            high: 0,
            low: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_password() {
        Password::new();
    }
}