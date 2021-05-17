use crate::Encoder;

pub enum EncodeType {
    Base32,
    Base32Hex,
}

pub struct Base32Encoder {
    encode_type: EncodeType,
}

impl Base32Encoder {
    pub fn new(encode_type: EncodeType) -> Self {
        Self { encode_type }
    }
}

impl Encoder for Base32Encoder {
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
