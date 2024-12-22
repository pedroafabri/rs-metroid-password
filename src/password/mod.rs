use crate::errors::BitOutOfBoundsError;

pub struct Password {
    bytes: [u8; 18]
}

impl Password {
    pub fn new() -> Password {
        Password {
            bytes: [0; 18]
        }
    }

    pub fn get_raw_bytes(&self) -> &[u8; 18] {
        &self.bytes
    }

    pub fn set_bit(&mut self, bit: u8) -> Result<(), BitOutOfBoundsError> {
        if bit > 127 { return Err(BitOutOfBoundsError); }
        
        let byte = self.get_byte_from_bit(&bit);
        let bit = self.get_relative_bit(&bit);
        let shifts = self.get_needed_shifts(&bit);

        self.bytes[byte as usize] |= 0x01 << shifts;
        
        Ok(())
    }

    pub fn clear_bit(&mut self, bit: u8) -> Result<(), BitOutOfBoundsError> {
        if bit > 127 { return Err(BitOutOfBoundsError); }

        let byte = self.get_byte_from_bit(&bit);
        let bit = self.get_relative_bit(&bit);
        let shifts = self.get_needed_shifts(&bit);

        self.bytes[byte as usize] &= 0xFE << shifts;

        Ok(())
    }

    fn get_byte_from_bit(&self, bit: &u8) -> u8 {
        bit / 8
    }

    fn get_relative_bit(&self, absolute_bit: &u8) -> u8 {
        let byte = self.get_byte_from_bit(&absolute_bit);
        absolute_bit - (byte * 8)
    }

    fn get_needed_shifts(&self, bit: &u8) -> u8 {
        7 - bit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_byte_from_bit() {
        let password = Password::new();

        let byte = password.get_byte_from_bit(&10);
        assert_eq!(byte, 1);

        let byte = password.get_byte_from_bit(&19);
        assert_eq!(byte, 2);

        let byte = password.get_byte_from_bit(&23);
        assert_eq!(byte, 2);
    }
}