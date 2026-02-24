const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn get_b64_value(character: char) -> Option<usize> {
    BASE64_ALPHABET.chars().position(|c| c == character)
}

fn get_b64_char(value: usize) -> Option<char> {
    BASE64_ALPHABET.chars().nth(value)
}

pub fn base64_decode(value: &str) -> String {
    let mut all_bits = String::new();
    for c in value.chars() {
        let value = if c == '=' {
            0
        } else {
            get_b64_value(c).unwrap()
        };
        let b6_bits = &format!("{:08b}", value)[2..];
        all_bits.push_str(b6_bits);
    }

    let mut all_bytes = vec![];
    let mut byte: u8 = 0;
    for (idx, bit) in all_bits.chars().enumerate() {
        let scaled_idx = idx % 8;
        if idx > 0 && idx % 8 == 0 {
            all_bytes.push(byte);
            byte = 0;
        }

        if bit == '1' {
            byte += 1 << (7 - scaled_idx);
        }
    }

    all_bytes.push(byte);
    all_bytes.retain(|&byte| byte != 0);

    str::from_utf8(&all_bytes).unwrap().to_owned()
}

pub fn base64_encode(value: &str) -> String {
    let mut bytes = value.bytes().collect::<Vec<_>>();
    let rem = bytes.len() % 3;

    let padding = match rem {
        1 => 2,
        2 => 1,
        _ => 0,
    };

    for _ in 0..padding {
        bytes.push(0);
    }

    let mut all_bits = vec![];
    for byte in bytes {
        let bits = format!("{:08b}", byte);
        for bit in bits.chars() {
            all_bits.push(bit);
        }
    }

    let mut b6_bytes = String::new();
    let mut b6_val = 0;
    let pad_starts = all_bits.len() - padding * 6;

    for (idx, bit) in all_bits.iter().enumerate() {
        let scaled_idx = idx % 6;
        if idx > 0 && scaled_idx == 0 {
            let char = if idx > pad_starts {
                '='
            } else {
                get_b64_char(b6_val).unwrap()
            };
            b6_bytes.push(char);
            b6_val = 0;
        }

        if *bit == '1' {
            b6_val += 1 << (5 - scaled_idx);
        }
    }

    let char = if rem > 0 {
        '='
    } else {
        get_b64_char(b6_val).unwrap()
    };

    b6_bytes.push(char);

    b6_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_ascii_1b() {
        let result = base64_encode("a");
        assert_eq!(result, "YQ==");
    }

    #[test]
    fn test_encode_island_2b() {
        let result = base64_encode("ö");
        assert_eq!(result, "w7Y=");
    }

    #[test]
    fn test_encode_kanji_3b() {
        let result = base64_encode("漢");
        assert_eq!(result, "5ryi");
    }

    #[test]
    fn test_encode_emoji_4b() {
        let result = base64_encode("💀");
        assert_eq!(result, "8J+SgA==");
    }

    #[test]
    fn test_decode_ascii_1b() {
        let result = base64_decode("YQ==");
        assert_eq!(result, "a");
    }

    #[test]
    fn test_decode_island_2b() {
        let result = base64_decode("w7Y=");
        assert_eq!(result, "ö");
    }

    #[test]
    fn test_decode_kanji_3b() {
        let result = base64_decode("5ryi");
        assert_eq!(result, "漢");
    }

    #[test]
    fn test_decode_emoji_4b() {
        let result = base64_decode("8J+SgA==");
        assert_eq!(result, "💀");
    }
}
