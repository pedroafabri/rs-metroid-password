#[cfg(test)]
mod tests {
    use metroid_password::errors::*;
    use metroid_password::password::*;

    #[test]
    fn should_create_new_password() {
        Password::new();
    }

    #[test]
    fn should_get_zeroed_raw_bytes() {
        let pass = Password::new();
        let bytes = pass.get_raw_bytes();
        let expected = [0u8; 18];
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn should_return_error_when_setting_byte_out_of_bounds() {
        let mut pass = Password::new();
        assert!(pass.set_byte(18, 0x0F).is_err());
    }

    #[test]
    fn should_set_whole_byte() {
        let mut pass = Password::new();
        assert!(!pass.set_byte(2, 0x0F).is_err());
        assert_eq!(pass.get_raw_bytes()[2], 0x0F);
    }

    #[test]
    fn should_return_error_when_setting_bit_out_of_bounds() {
        let mut pass = Password::new();
        let r = pass.set_bit(128);
        assert!(matches!(r, Err(BitOutOfBoundsError)));
    }

    #[test]
    fn should_set_last_bit_of_byte() {
        let mut pass = Password::new();
        let r = pass.set_bit(127);
        assert!(!r.is_err());
        let bytes = pass.get_raw_bytes();
        assert_eq!(bytes[15], 1);
    }

    #[test]
    fn should_set_first_and_last_bits_of_byte() {
        let mut pass = Password::new();
        assert!(!pass.set_bit(127).is_err());
        assert!(!pass.set_bit(120).is_err());
        assert_eq!(pass.get_raw_bytes()[15], 129);
    }

    #[test]
    fn should_clear_last_bit_of_byte() {
        let mut pass = Password::new();
        assert!(!pass.set_bit(127).is_err());
        assert!(!pass.clear_bit(127).is_err());
        assert_eq!(pass.get_raw_bytes()[15], 0);
    }

}