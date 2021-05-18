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
    fn test_base64_encode() {
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
            "Zg==",
            "Zm8=",
            "Zm9v",
            "Zm9vYg==",
            "Zm9vYmE=",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB+",
        ];
        let encoder = Base64Encoder::new(EncodeType::Base64);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base64_decode() {
        let test = vec![
            "",
            "Zg==",
            "Zm8=",
            "Zm9v",
            "Zm9vYg==",
            "Zm9vYmE=",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB+",
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
        let encoder = Base64Encoder::new(EncodeType::Base64);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }

    #[test]
    fn test_base64url_encode() {
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
            "Zg",
            "Zm8",
            "Zm9v",
            "Zm9vYg",
            "Zm9vYmE",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB-",
        ];
        let encoder = Base64Encoder::new(EncodeType::Base64Url);
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base64url_decode() {
        let test = vec![
            "",
            "Zg",
            "Zm8",
            "Zm9v",
            "Zm9vYg",
            "Zm9vYmE",
            "Zm9vYmFy",
            "YWJjMTIzIT8kKiYoKSctPUB-",
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
        let encoder = Base64Encoder::new(EncodeType::Base64Url);
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }
}
