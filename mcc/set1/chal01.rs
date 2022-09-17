use std::{fmt, num::ParseIntError};

extern crate base64;
extern crate hex;

// Convert hex string from ASCII / UTF-8 to raw bytes
#[allow(dead_code)]
pub fn decode_hex_custom(hex: &str) -> Result<Vec<u8>, DecodeHexError> {

    // Check for ill-formed hex string
    if hex.len() % 2 == 1 {
        Err(DecodeHexError::OddLength)
    }
    else {
        // Increment by 2-nibble bytes, interpret as hex, then convert to raw bytes (u8) and collect
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i+2], 16).map_err(|e| e.into()))
            .collect()
    }
}

// Receive a single chunk of 1-3 bytes (by assumption), then re-chunk into 6-bit values
#[allow(dead_code)]
fn split_b64_custom(byte_chunk: Vec<u8>) -> Vec<u8> {

    // 24 b chunk of bytes [x,y,z], assuming remainder (if any) is 0
    let x = byte_chunk[0];
    let y = *byte_chunk.get(1).unwrap_or(&0);
    let z = *byte_chunk.get(2).unwrap_or(&0);

    // 6 b chunks of b64 [a,b,c,d], again assmuming remainder is 0
    let a: u8 = x >> 2;
    let b: u8 = ((x & 0b00000011) << 4) | (y >> 4);
    let c: u8 = ((y & 0b00001111) << 2) | (z >> 6);
    let d: u8 = z & 0b00111111;

    vec![a, b, c, d]
}

// Encode raw bytes to base64 by re-chunking 8 b -> 24 b -> 6 b (i.e. 0..63)
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn encode_b64_custom(bytes: Vec<u8>) -> Vec<u8> {

    // TODO: I can't justify the time doing boring bit manipulation for hours just to
    // coalesce from raw bytes into base64-encoded characters when many tools already
    // exist to do this, but here's the high-level algorithm:
    //      1. Collect bytes (8-bit) into 6-bit -> 24-bit "chunks"
    //      2. Encode each 24-bit chunk as 1-4 base-64 characters (see `split_b64_custom`)
    //      3. Append 0s and add padding (=) as necessary
    Vec::<u8>::new()
}

// Use standard base64 encoding, where 0..63 maps to <A-Z><a-z><0-9>+/ and = is padding character
#[allow(dead_code)]
pub fn b64_to_char(bc: u8) -> char {
    match bc {
        // 'A' is base64 offset 0 (-), ASCII offset 65 (through 'Z')
        0..=25  => (bc + 65) as char,
        // 'a' is base64 offset 26 (-), ASCII offset 97 (through 'z')
        26..=51 => (bc + 71) as char,
        // 0 is base64 offset 52 (-), ASCII offset 48 (through 9)
        52..=61 => (bc-4) as char,
        62      => '+',
        63      => '/',
        _       => unreachable!()
    }
}

// Custom DecodeHexError
// NOTE: I'm being pedantic about proper error checking even though I'll probably never need this
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeHexError {
    OddLength,
    ParseInt(ParseIntError),
}
impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}
impl fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeHexError::OddLength   => "cannot decode odd-length hex string to bytes".fmt(f),
            DecodeHexError::ParseInt(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for DecodeHexError {}


// Decode hex to bytes then encode with standard base64 encoding from scratch
#[allow(dead_code)]
fn hex_to_b64str_custom(hex: &str) -> String {

    let _bytes = decode_hex_custom(hex).unwrap();
//    println!("{}", String::from_utf8(bytes).unwrap());

    String::from("")
}

pub fn hex_to_b64(hex: &str) -> String {
    let bytes = match hex::decode(hex) {
        Ok(blob)    => blob,
        Err(e)      => panic!("Could not decode hex string: {:?}", e),
    };
    base64::encode(bytes)
}


// --------------------------------
// | TESTS                        |
// --------------------------------

#[cfg(test)]
mod set01_chal01_tests {

    use super::*;

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(
            hex_to_b64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
