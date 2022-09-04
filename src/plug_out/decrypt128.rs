use super::common::{
    add_round_key,
    decrypt::inverse_round,
};

pub const fn decrypt128(key: [u8; 16], data: [u8; 8]) -> [u8; 8] {
    // Convert from BE to LE format.
    let round_keys: [[u8; 8]; 2] = [
        [ key[15], key[14], key[13], key[12], key[11], key[10], key[ 9], key[ 8] ],
        [ key[ 7], key[ 6], key[ 5], key[ 4], key[ 3], key[ 2], key[ 1], key[ 0] ],
    ];
    let mut state = [ data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0] ];
    state = inverse_round(state, round_keys[1], 13);
    state = inverse_round(state, round_keys[0], 12);
    state = inverse_round(state, round_keys[1], 11);
    state = inverse_round(state, round_keys[0], 10);
    state = inverse_round(state, round_keys[1], 9);
    state = inverse_round(state, round_keys[0], 8);
    state = inverse_round(state, round_keys[1], 7);
    state = inverse_round(state, round_keys[0], 6);
    state = inverse_round(state, round_keys[1], 5);
    state = inverse_round(state, round_keys[0], 4);
    state = inverse_round(state, round_keys[1], 3);
    state = inverse_round(state, round_keys[0], 2);
    state = inverse_round(state, round_keys[1], 1);
    state = add_round_key(state, round_keys[0], 0);
    // Convert from LE to BE format.
    [ state[7], state[6], state[5], state[4], state[3], state[2], state[1], state[0] ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        const KEY: [u8; 16] = [
            0x6D, 0xC4, 0x16, 0xDD, 0x77, 0x94, 0x28, 0xD2, 0x7E, 0x1D, 0x20, 0xAD, 0x2E, 0x15, 0x22, 0x97,
        ];
        const ENCRYPTED_DATA: [u8; 8] = [
            0x6B, 0x6B, 0x29, 0x81, 0xAD, 0x5D, 0x03, 0x27,
        ];
        const EXPECTED_DATA: [u8; 8] = [
            0x09, 0x85, 0x52, 0xF6, 0x1E, 0x27, 0x00, 0x26,
        ];
        const DECRYPTED_DATA: [u8; 8] = decrypt128(KEY, ENCRYPTED_DATA);
        assert_eq!(DECRYPTED_DATA, EXPECTED_DATA);
    }
}
