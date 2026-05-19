const ENGLISH_FREQ: [f64; 26] = [
    8.17, 1.49, 2.78, 4.25, 12.70, 2.23, 2.02, 6.09, 6.97, 0.15, 0.77, 4.03, 2.41, 6.75, 7.51,
    1.93, 0.10, 5.99, 6.33, 9.06, 2.76, 0.98, 2.36, 0.15, 1.97, 0.07,
];

fn chi_squared(column: &[u8], shift: usize) -> f64 {
    let mut freq = [0usize; 26];
    for &byte in column {
        freq[(byte - b'A') as usize] += 1;
    }
    let n = column.len() as f64;
    (0..26)
        .map(|i| {
            let expected = n * ENGLISH_FREQ[(i + 26 - shift) % 26] / 100.0;
            if expected > 0.0 {
                let diff = freq[i] as f64 - expected;
                diff * diff / expected
            } else {
                0.0
            }
        })
        .sum()
}

pub fn decipher_key(ciphertext: &str, key_length: usize) -> String {
    let bytes: Vec<u8> = ciphertext
        .bytes()
        .filter(|b| b.is_ascii_uppercase())
        .collect();

    (0..key_length)
        .map(|offset| {
            // Extract every key_length-th character starting at `offset`
            let column: Vec<u8> = bytes
                .iter()
                .skip(offset)
                .step_by(key_length)
                .copied()
                .collect();

            // Find the shift (0–25) that minimises χ² against English frequencies
            let best_shift = (0..26)
                .min_by(|&a, &b| {
                    chi_squared(&column, a)
                        .partial_cmp(&chi_squared(&column, b))
                        .unwrap()
                })
                .unwrap();

            (b'A' + best_shift as u8) as char
        })
        .collect()
}

fn main() {
    println!("{}", decipher_key("HFNIMVOSNA", 6));
    println!("{}", decipher_key("LXFOPVEFRNHR", 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_wikipedia() {
        // Encrypt "HELLOWORLD" with key "ABCXYZ" → "HFNIMVOSNA"
        assert_eq!(decipher_key("HFNIMVOSNA", 6), "ABCXYZ");
    }

    #[test]
    fn longer_text() {
        // The longer the ciphertext relative to key_length, the more reliable
        let ct = "LXFOPVEFRNHR"; // "ATTACKATDAWN" with key "LEMON"
        assert_eq!(decipher_key(ct, 5), "LEMON");
    }
}
