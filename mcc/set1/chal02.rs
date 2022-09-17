extern crate hex;

pub fn fixed_xor(x: &str, y: &str) -> String {

    let x_bytes = match hex::decode(x) {
        Ok(blob)    => blob,
        Err(e)      => panic!("Could not decode 1st hex string: {:?}", e),
    };
    let y_bytes = match hex::decode(y) {
        Ok(blob)    => blob,
        Err(e)      => panic!("Could not decode 2nd hex string: {:?}", e),
    };

    assert_eq!(x_bytes.len(), y_bytes.len(), "bitstrings are not the same size!");

    let xor_bits = x_bytes.iter().zip(y_bytes.iter())
        .map(|(xb, yb)| xb ^ yb )
        .collect::<Vec<u8>>();

    hex::encode(xor_bits)
}

// --------------------------------
// | TESTS                        |
// --------------------------------

#[cfg(test)]
mod set01_chal02_tests {

    use super::*;

    #[test]
    fn test_fixed_xor() {
        assert_eq!(
            fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"),
            "746865206b696420646f6e277420706c6179"
        );
    }
}
