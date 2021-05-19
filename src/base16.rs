use crate::Encoder;

const BASE16: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
const BASE16_STR: &str = "0123456789ABCDEF";

pub struct Base16Encoder {}

impl Base16Encoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for Base16Encoder {
    fn encode(&self, input: &[u8]) -> String {
        let mut bits = String::new();
        input.iter().for_each(|byte| {
            bits = format!("{}{:08b}", bits, byte);
        });
        let mut left: usize = 0;
        let last = bits.len();
        let mut result = String::new();

        while left < last {
            let right = if left.saturating_add(4) > last {
                last
            } else {
                left.saturating_add(4)
            };
            let pos = usize::from_str_radix(&format!("{:0<4}", &bits[left..right]), 2).unwrap();
            let ch = BASE16[pos];
            result = format!("{}{}", result, ch);
            left = right;
        }

        result
    }

    fn decode(&self, input: &str) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        let mut bits = String::new();

        for c in input.chars() {
            let base16_pos = BASE16_STR.find(c);
            if let Some(pos) = base16_pos {
                bits = format!("{}{:0>4b}", bits, pos);
            } else {
                bits = format!("{}0000", bits);
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
    fn test_base16_encode() {
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
            "66",
            "666F",
            "666F6F",
            "666F6F62",
            "666F6F6261",
            "666F6F626172",
            "616263313233213F242A262829272D3D407E",
        ];
        let encoder = Base16Encoder::new();
        test_bytes.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect[i], encoder.encode(t));
        });
    }

    #[test]
    fn test_base16_decode() {
        let test = vec![
            "",
            "66",
            "666F",
            "666F6F",
            "666F6F62",
            "666F6F6261",
            "666F6F626172",
            "616263313233213F242A262829272D3D407E",
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
        let encoder = Base16Encoder::new();
        test.iter().enumerate().for_each(|(i, t)| {
            assert_eq!(expect_bytes[i], encoder.decode(t));
        });
    }
}
