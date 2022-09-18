use hex;

/// Return a string slice with a hex prefix
pub fn add_0x(s: &str) -> &str {
    if s.len() > 2 && &s[0..2] == "0x" {
        return s;
    }

    "0x{s}"
}

/// Return a slice of a hex string without the prefix
pub fn remove_0x(s: &str) -> &str {
    if s.len() <= 2 {
        return s;
    } else if &s[0..2] == "0x" {
        return &s[2..];
    }

    s
}

/// Decode a byte array from a hex color string
pub fn decode_hex_color(s: &str) -> [u8; 4] {
    assert!(s.len() >= 6);

    [
        hex::decode(&s[0..2]).unwrap()[0],
        hex::decode(&s[2..4]).unwrap()[0],
        hex::decode(&s[4..6]).unwrap()[0],
        if s.len() > 6 {
            hex::decode(&s[6..8]).unwrap()[0]
        } else {
            255
        },
    ]
}

/// Chunk a hex string into a vector of hex bytes (e.g. `["ff", "ef"]`)
pub fn chunk_hex_str(s: &str) -> Vec<&str> {
    assert!(s.len() % 2 == 0);

    let chunk_count = s.len() / 2;
    let range = 0..chunk_count;

    range.into_iter().map(|i| &s[i..i + 2]).collect()
}
