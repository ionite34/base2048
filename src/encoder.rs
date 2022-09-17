use std::fmt::Error;
/// Original Implementation by [LLFourn](https://github.com/LLFourn/rust-base2048),
/// based on [qntm/base2048](https://github.com/qntm/base2048)
use hashbrown::HashSet;
use crate::DecodeError;

pub const ENC_TABLE: [char; 2048] = include!("./enc_table.src");
pub const DEC_TABLE: [u16; 4340] = include!("./dec_table.src");
pub const TAIL: [char; 8] = ['།', '༎', '༏', '༐', '༑', '༆', '༈', '༒'];

// Create a hash set of all index in DEC_TABLE that map to 0xFFFF
lazy_static! {
    static ref ZERO_SET: HashSet<u16> = {
        let mut set = HashSet::new();
        for (i, &v) in DEC_TABLE.iter().enumerate() {
            if v == 0xFFFF {
                set.insert(i as u16);
            }
        }
        set
    };
}

/// Encode bytes using base2048
pub fn encode(bytes: &[u8]) -> String {
    let mut ret = String::new();
    let mut stage = 0x0000u16;
    let mut remaining = 0;

    for byte in bytes {
        let byte = *byte as u16;
        // how many more bits do we need to complete the next character?
        let need = 11 - remaining;
        if need <= 8 {
            // if we need a byte or less then take what we need and push it
            remaining = 8 - need;
            let index = (stage << need) | (byte >> remaining);
            ret.push(ENC_TABLE[index as usize]);
            // put what remains in stage
            stage = byte & ((1 << remaining) - 1);
        } else {
            // we need more than a byte so just shift it into stage
            stage = (stage << 8) | byte;
            remaining += 8;
        }
    }

    // there are some bits that haven't been put into the string
    // (happens whenever 8 * bytes.len() is not divisible by 11).
    if remaining > 0 {
        // We need to disambiguate between a terminating character conveying =< 3 or > 8 bits.
        // e.g. is this character just finishing the last byte or is it doing that and adding another byte.
        if remaining <= 3 {
            // we're adding 1-3 bits so add special tail character
            ret.push(TAIL[stage as usize]);
        } else {
            // we're adding > 3 bits no need for a tail since it's not ambiguous
            ret.push(ENC_TABLE[stage as usize])
        }
    }

    ret
}

/// Decode a base2048 encoded string into bytes
pub fn decode(string: &str) -> Result<Vec<u8>, String> {
    let mut ret = vec![];
    let mut remaining = 0u8;
    let mut stage = 0x00u32;
    let mut chars = string.chars().enumerate().peekable();
    let mut residue = 0;

    while let Some((i, c)) = chars.next() {
        // keep track of the misalignment between byte boundary.  This is useful when we get to the
        // last character and it's NOT a tail character.
        residue = (residue + 11) % 8;

        let (n_new_bits, new_bits) = match c {
            // Check if the character is in the zero set
            c if ZERO_SET.contains(&(c as u16)) => {
                if let Some((i_next, c_next)) = chars.peek() {
                    return Err(format!("Unexpected character {i_next}: [{c_next:?}] after termination sequence {i}: [{c:?}]"));
                }

                match TAIL.iter().enumerate().find(|(_, t)| *t == &c) {
                    // so we're at the last character and it's a tail character
                    Some((index, _)) => {
                        let need = 8 - remaining;
                        if index < (1 << need) {
                            (need, index as u16)
                        } else {
                            return Err(format!("Invalid tail character {i}: [{c:?}]"));
                        }
                    }
                    None => return Err(format!("Invalid termination character {i}: [{c:?}]")),
                }
            }
            // Reference Decode Table
            _ => {
                let new_bits = DEC_TABLE[c as usize] as u16;
                match chars.peek() {
                    None => { (11 - residue, new_bits) }
                    Some(_) => { (11, new_bits) }
                }
            },
        };

        remaining += n_new_bits;
        stage = (stage << n_new_bits) | new_bits as u32;
        while remaining >= 8 {
            //NOTE: This loop runs at most twice
            remaining -= 8;
            ret.push((stage >> remaining) as u8);
            stage &= (1 << remaining) - 1
        }
    }

    if remaining > 0 {
        ret.push((stage >> (8 - remaining)) as u8)
    }

    Ok(ret)
}

//noinspection SpellCheckingInspection
#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(b"Hello", "ϓțƘį")]
    fn test_encode(#[case] input: &[u8], #[case] expected: String) {
        assert_eq!(expected, encode(input));
    }

    #[rstest]
    fn test_chars() {
        for i in 0..=u16::MAX {
            let two_bytes = i.to_be_bytes();
            let encoded = encode(&two_bytes[..]);
            let decoded = decode(&encoded).unwrap();
            assert_eq!(two_bytes[..], decoded[..]);
        }
    }

    #[rstest]
    #[case("ետћζы༎", b"x/me\"eu")]
    fn test_decode(#[case] input: &str, #[case] expected: &[u8]) {
        assert_eq!(expected, &decode(input).unwrap());
    }

    #[rstest]
    // this is a valid tail character but conveys too many bits.
    #[case("ետћζы༑", "Invalid tail character 5: ['༑']")]
    // Invalid because of the X at the end
    #[case("ետћζыX", "Invalid termination character 5: ['X']")]
    #[case("ետћζы༎X", "Unexpected character 6: ['X'] after termination sequence 5: ['༎']")]
    fn test_decode_invalid(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(decode(input).unwrap_err(), expected);
    }
}
