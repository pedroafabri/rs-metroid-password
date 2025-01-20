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

    pub fn get_raw_bytes(&self) -> (u128, u16) {
        (self.high, self.low)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_password() {
        let pass = Password::new();
        assert_eq!(pass.high, 0);
        assert_eq!(pass.low, 0);
    }

    #[test]
    fn should_return_correct_raw_bytes() {
        let mut pass = Password::new();
        pass.high = 3;
        pass.low = 7;
        let raw = pass.get_raw_bytes();
        assert_eq!(raw, (3, 7));
    }
}