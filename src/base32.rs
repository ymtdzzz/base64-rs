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
    fn encode(&self, _input: &[u8]) -> String {
        String::default()
    }

    fn decode(&self, _input: &str) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_encode() {
        let test = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let test_bytes: Vec<&[u8]> = test.iter().map(|t| t.as_bytes()).collect();
        let expect = vec![
            "",
            "MY======",
            "MZXQ====",
            "MZXW6===",
            "MZXW6YQ=",
            "MZXW6YTB",
            "MZXW6YTBOI======",
            "MFRGGMJSGMQT6JBKEYUCSJZNHVAH4===",
        ];
        let encoder = Base32Encoder::new(EncodeType::Base32);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base32_decode() {
        let test = vec![
            "",
            "MY======",
            "MZXQ====",
            "MZXW6===",
            "MZXW6YQ=",
            "MZXW6YTB",
            "MZXW6YTBOI======",
            "MFRGGMJSGMQT6JBKEYUCSJZNHVAH4===",
        ];
        let expect = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let expect_bytes: Vec<&[u8]> = expect.iter().map(|e| e.as_bytes()).collect();
        let encoder = Base32Encoder::new(EncodeType::Base32);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }

    #[test]
    fn test_base32hex_encode() {
        let test = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let test_bytes: Vec<&[u8]> = test.iter().map(|t| t.as_bytes()).collect();
        let expect = vec![
            "",
            "CO======",
            "CPNG====",
            "CPNMU===",
            "CPNMUOG=",
            "CPNMUOJ1",
            "CPNMUOJ1E8======",
            "C5H66C9I6CGJU91A4OK2I9PD7L07S===",
        ];
        let encoder = Base32Encoder::new(EncodeType::Base32Hex);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base32hex_decode() {
        let test = vec![
            "",
            "CO======",
            "CPNG====",
            "CPNMU===",
            "CPNMUOG=",
            "CPNMUOJ1",
            "CPNMUOJ1E8======",
            "C5H66C9I6CGJU91A4OK2I9PD7L07S===",
        ];
        let expect = vec![
            "",
            "f",
            "fo",
            "foo",
            "foob",
            "fooba",
            "foobar",
            "abc123!?$*&()'-=@~",
        ];
        let expect_bytes: Vec<&[u8]> = expect.iter().map(|e| e.as_bytes()).collect();
        let encoder = Base32Encoder::new(EncodeType::Base32Hex);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }
}
