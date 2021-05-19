use crate::Encoder;

const BASE32: [char; 32] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7',
];
const BASE32_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const BASE32HEX: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
];
const BASE32HEX_STR: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUV";

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
    fn encode(&self, input: &[u8]) -> String {
        let mut bits = String::new();
        input.iter().for_each(|byte| {
            bits = format!("{}{:08b}", bits, byte);
        });
        let mut left: usize = 0;
        let last = bits.len();
        let mut result = String::new();

        while left < last {
            let right = if left.saturating_add(5) > last {
                last
            } else {
                left.saturating_add(5)
            };
            let pos = usize::from_str_radix(&format!("{:0<5}", &bits[left..right]), 2).unwrap();
            let ch = match self.encode_type {
                EncodeType::Base32 => BASE32[pos],
                EncodeType::Base32Hex => BASE32HEX[pos],
            };
            result = format!("{}{}", result, ch);
            left = right;
        }

        if result.len() % 8 > 0 {
            for _ in 0..(8 - (result.len() % 8)) {
                result = format!("{}=", result);
            }
        }

        result
    }

    fn decode(&self, input: &str) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        let mut bits = String::new();

        for c in input.chars() {
            let base32_pos = match self.encode_type {
                EncodeType::Base32 => BASE32_STR.find(c),
                EncodeType::Base32Hex => BASE32HEX_STR.find(c),
            };
            if let Some(pos) = base32_pos {
                bits = format!("{}{:0>5b}", bits, pos);
            } else {
                bits = format!("{}00000", bits);
            }
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
