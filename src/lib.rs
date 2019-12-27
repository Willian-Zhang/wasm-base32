use wasm_bindgen::prelude::*;
use base32;
use hex;

#[wasm_bindgen]
pub fn decode_to_string(encoded: String) -> Option<String> {
    return match base32::decode(base32::Alphabet::RFC4648{padding: false}, &encoded) {
        Some(vec) => Some(hex::encode(&vec)),
        None => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_to_string() {
        assert_eq!(decode_to_string(
            String::from("HB2NB2OISKNO3347UQVL7Y5VZV43WBDS")), 
            Some(String::from("3874d0e9c8929aedef9fa42abfe3b5cd79bb0472"))
        );
    }
}