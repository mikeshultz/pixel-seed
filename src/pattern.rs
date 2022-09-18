//! # Pattern Utilities
//!
//!  Utilities to build drawing patterns

use bytes::{BufMut, BytesMut};
use hex;
use sha2::{Digest, Sha256};

/// SHA256 hash a set of bytes
fn hash_bytes(b: &BytesMut) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(b);
    hasher.finalize().to_vec()
}

/// Given a portion of the seed and a target pattern size, generate a pattern.
/// A pattern is a set of hex bytes where each byte is used to figure out if a
/// pixel should be "on" (fg vs bg), and which direction to travel next.
pub fn seed_to_pattern(seed_part: &str, target_size_bytes: u32) -> String {
    let mut consumed = 0;
    let mut pattern = BytesMut::with_capacity(target_size_bytes as usize);

    for byt in String::from(seed_part).into_bytes() {
        pattern.put_u8(byt);
        consumed += 1;

        if consumed < target_size_bytes {
            pattern.put_u8(byt);
            consumed += 1;
        } else {
            eprintln!("Pattern quite short.");
            break;
        }
    }
    'outer: loop {
        let hashed = hash_bytes(&pattern);

        for byt in hashed {
            if consumed < target_size_bytes {
                pattern.put_u8(byt);
                consumed += 1;
            } else {
                break 'outer;
            }
        }
    }

    hex::encode(pattern)
}
