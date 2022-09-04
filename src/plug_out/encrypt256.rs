use super::common::{
    add_round_key,
    encrypt::round,
};

pub const fn encrypt256(key: [u8; 32], data: [u8; 8]) -> [u8; 8] {
    // Convert from BE to LE format.
    let round_keys: [[u8; 8]; 4] = [
        [ key[31], key[30], key[29], key[28], key[27], key[26], key[25], key[24] ],
        [ key[23], key[22], key[21], key[20], key[19], key[18], key[17], key[16] ],
        [ key[15], key[14], key[13], key[12], key[11], key[10], key[ 9], key[ 8] ],
        [ key[ 7], key[ 6], key[ 5], key[ 4], key[ 3], key[ 2], key[ 1], key[ 0] ],
    ];
    let mut state = [ data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0] ];
    state = add_round_key(state, round_keys[0], 0);
    state = round(state, round_keys[1], 1);
    state = round(state, round_keys[2], 2);
    state = round(state, round_keys[3], 3);
    state = round(state, round_keys[0], 4);
    state = round(state, round_keys[1], 5);
    state = round(state, round_keys[2], 6);
    state = round(state, round_keys[3], 7);
    state = round(state, round_keys[0], 8);
    state = round(state, round_keys[1], 9);
    state = round(state, round_keys[2], 10);
    state = round(state, round_keys[3], 11);
    state = round(state, round_keys[0], 12);
    state = round(state, round_keys[1], 13);
    state = round(state, round_keys[2], 14);
    state = round(state, round_keys[3], 15);
    state = round(state, round_keys[0], 16);
    state = round(state, round_keys[1], 17);
    // Convert back from BE to LE format.
    [ state[7], state[6], state[5], state[4], state[3], state[2], state[1], state[0] ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        const KEY: [u8; 32] = [
            0x00, 0x9A, 0x3A, 0xA4, 0x76, 0xA9, 0x6D, 0xB5, 0x54, 0xA7, 0x12, 0x06, 0x26, 0xD1, 0x56, 0x33,
            0x6D, 0xC4, 0x16, 0xDD, 0x77, 0x94, 0x28, 0xD2, 0x7E, 0x1D, 0x20, 0xAD, 0x2E, 0x15, 0x22, 0x97,
        ];
        const DATA: [u8; 8] = [
            0x09, 0x85, 0x52, 0xF6, 0x1E, 0x27, 0x00, 0x26,
        ];
        const EXPECTED_DATA: [u8; 8] = [
            0x81, 0x6D, 0xAE, 0x6F, 0xB6, 0x52, 0x38, 0x89,
        ];
        const ENCRYPTED_DATA: [u8; 8] = encrypt256(KEY, DATA);
        assert_eq!(ENCRYPTED_DATA, EXPECTED_DATA);
    }
}
