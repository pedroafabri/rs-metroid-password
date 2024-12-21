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
    fn should_return_error_when_setting_bit_out_of_bounds() {
        let mut pass = Password::new();
        let r = pass.set_bit(128);
        assert!(matches!(r, Err(BitOutOfBoundsError)));
    }

    #[test]
    fn should_set_last_bit_to_1() {
        let mut pass = Password::new();
        let r = pass.set_bit(127);
        assert!(!r.is_err());
        let bytes = pass.get_raw_bytes();
        assert_eq!(bytes[15], 1);
    }
}