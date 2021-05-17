use crate::Encoder;

pub enum EncodeType {
    Base64,
    Base64Url,
}

pub struct Base64Encoder {
    encode_type: EncodeType,
}

impl Base64Encoder {
    pub fn new(encode_type: EncodeType) -> Self {
        Self { encode_type }
    }
}

impl Encoder for Base64Encoder {
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
