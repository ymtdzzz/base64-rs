use std::str;

pub mod base16;
pub mod base32;
pub mod base64;

pub trait Encoder {
    fn encode(&self, input: &[u8]) -> String;
    fn decode(&self, input: &str) -> Vec<u8>;
}
