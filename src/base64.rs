use crate::Encoder;

const BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const BASE64_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64URL: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '-', '_',
];
const BASE64URL_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

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
    fn encode(&self, input: &[u8]) -> String {
        let mut bits = String::new();
        input.iter().for_each(|byte| {
            bits = format!("{}{:08b}", bits, byte);
        });
        let mut left: usize = 0;
        let last = bits.len();
        let mut result = String::new();
        let mut is_base64 = true;

        while left < last {
            let right = if left.saturating_add(6) > last {
                last
            } else {
                left.saturating_add(6)
            };
            let pos = usize::from_str_radix(&format!("{:0<6}", &bits[left..right]), 2).unwrap();
            let ch = match self.encode_type {
                EncodeType::Base64 => BASE64[pos],
                EncodeType::Base64Url => {
                    is_base64 = false;
                    BASE64URL[pos]
                }
            };
            result = format!("{}{}", result, ch);
            left = right;
        }

        if result.len() % 4 > 0 && is_base64 {
            for _ in 0..(4 - (result.len() % 4)) {
                result = format!("{}=", result);
            }
        }

        result
    }

    fn decode(&self, input: &str) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        let mut bits = String::new();

        for c in input.chars() {
            let base64_pos = match self.encode_type {
                EncodeType::Base64 => BASE64_STR.find(c),
                EncodeType::Base64Url => BASE64URL_STR.find(c),
            };
            bits = format!("{}{:0>6b}", bits, base64_pos.unwrap_or(0));
        }

        let mut idx: usize = 0;
        while idx < bits.len() {
            let end = idx.saturating_add(8);
            let b = if end <= bits.len() {
                bits[idx..end].to_string()
            } else {
                format!("{:0<8}", &bits[idx..(bits.len() - 1)])
            };
            let u8 = u8::from_str_radix(&b, 2).unwrap();
            if u8 != 0 {
                result.push(u8);
            }
            idx = idx.saturating_add(8);
        }

        result
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
