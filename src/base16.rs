use crate::Encoder;

pub struct Base16Encoder {}

impl Base16Encoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for Base16Encoder {
    fn encode(&self, input: &str) -> String {
        String::from(input)
    }

    fn decode(&self, input: &str) -> String {
        String::from(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
