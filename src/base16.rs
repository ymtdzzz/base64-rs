use crate::Encoder;

pub struct Base16Encoder {}

impl Base16Encoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for Base16Encoder {
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
