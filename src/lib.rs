use std::str;

pub mod base16;
pub mod base32;
pub mod base64;

pub trait Encoder {
    fn encode(&self, input: &str) -> String;
    fn decode(&self, input: &str) -> String;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
