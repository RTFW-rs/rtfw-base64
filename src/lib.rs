const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

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

fn get_b64_char(index: usize) -> Option<char> {
    BASE64_ALPHABET.chars().nth(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_1b() {
        let result = base64_encode("a");
        assert_eq!(result, "YQ==");
    }

    #[test]
    fn test_island_2b() {
        let result = base64_encode("ö");
        assert_eq!(result, "w7Y=");
    }

    #[test]
    fn test_kanji_3b() {
        let result = base64_encode("漢");
        assert_eq!(result, "5ryi");
    }

    #[test]
    fn test_emoji_4b() {
        let result = base64_encode("💀");
        assert_eq!(result, "8J+SgA==");
    }
}
