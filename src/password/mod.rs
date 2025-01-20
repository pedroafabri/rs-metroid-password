use crate::errors::{BitOutOfBoundsError, ByteOutOfBoundsError};

/// 144 bits NES Metroid password
pub struct Password {
    low: u128,
    high: u16
}

impl Password {
    /// Generates a new Password with zeroed bits
    pub fn new() -> Password {
        Password {
            low: 0,
            high: 0
        }
    }

    /// Gets the raw bytes of the password.
    /// 
    /// It returns a `(u16, u128)` representing the `(high_byte, low_byte)` of the Password
    pub fn get_raw_bytes(&self) -> (u16, u128) {
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