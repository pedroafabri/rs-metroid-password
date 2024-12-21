//! NES Metroid Password Generator
//!
//! This library provides functionality to generate and manipulate passwords for NES Metroid.
//! The Metroid password consists of 24 characters from the Metroid Alphabet, representing
//! 144 bits of data. With this library, you can modify individual bits within the password's
//! 144-bit structure and encode the resulting password into a human-readable format that can
//! be used directly in the game.

pub mod password;
pub mod errors;