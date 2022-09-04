pub fn encrypt(key: &[u8; 16], data: &[u8; 8]) -> [u8; 8] {
    // Convert from BE to LE format.
    let round_keys: [[u8; 8]; 2] = [
        [ key[15], key[14], key[13], key[12], key[11], key[10], key[ 9], key[ 8] ],
        [ key[ 7], key[ 6], key[ 5], key[ 4], key[ 3], key[ 2], key[ 1], key[ 0] ],
    ];
    let mut state = [ data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0] ];
    add_round_key(&mut state, &round_keys[0], 0);
    round(&mut state, &round_keys[1],  1);
    round(&mut state, &round_keys[0],  2);
    round(&mut state, &round_keys[1],  3);
    round(&mut state, &round_keys[0],  4);
    round(&mut state, &round_keys[1],  5);
    round(&mut state, &round_keys[0],  6);
    round(&mut state, &round_keys[1],  7);
    round(&mut state, &round_keys[0],  8);
    round(&mut state, &round_keys[1],  9);
    round(&mut state, &round_keys[0], 10);
    round(&mut state, &round_keys[1], 11);
    round(&mut state, &round_keys[0], 12);
    round(&mut state, &round_keys[1], 13);
    // Convert back from BE to LE format.
    [ state[7], state[6], state[5], state[4], state[3], state[2], state[1], state[0] ]
}

pub fn decrypt(key: &[u8; 16], data: &[u8; 8]) -> [u8; 8] {
    // Convert from BE to LE format.
    let round_keys: [[u8; 8]; 2] = [
        [ key[15], key[14], key[13], key[12], key[11], key[10], key[ 9], key[ 8] ],
        [ key[ 7], key[ 6], key[ 5], key[ 4], key[ 3], key[ 2], key[ 1], key[ 0] ],
    ];
    let mut state = [ data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0] ];
    inverse_round(&mut state, &round_keys[1], 13);
    inverse_round(&mut state, &round_keys[0], 12);
    inverse_round(&mut state, &round_keys[1], 11);
    inverse_round(&mut state, &round_keys[0], 10);
    inverse_round(&mut state, &round_keys[1],  9);
    inverse_round(&mut state, &round_keys[0],  8);
    inverse_round(&mut state, &round_keys[1],  7);
    inverse_round(&mut state, &round_keys[0],  6);
    inverse_round(&mut state, &round_keys[1],  5);
    inverse_round(&mut state, &round_keys[0],  4);
    inverse_round(&mut state, &round_keys[1],  3);
    inverse_round(&mut state, &round_keys[0],  2);
    inverse_round(&mut state, &round_keys[1],  1);
    add_round_key(&mut state, &round_keys[0], 0);
    // Convert from LE to BE format.
    [ state[7], state[6], state[5], state[4], state[3], state[2], state[1], state[0] ]
}

#[inline(always)]
fn round(state: &mut [u8; 8], round_key: &[u8; 8], round: u8) {
    s_box(state);
    rotate(state);
    add_round_key(state, round_key, round);
}

#[inline(always)]
fn add_round_key(state: &mut [u8; 8], round_key: &[u8; 8], round: u8) {
    state[0] ^= round_key[0] ^ round;
    state[1] ^= round_key[1];
    state[2] ^= round_key[2];
    state[3] ^= round_key[3];
    state[4] ^= round_key[4];
    state[5] ^= round_key[5];
    state[6] ^= round_key[6];
    state[7] ^= round_key[7];
}

#[inline(always)]
fn s_box(x: &mut [u8; 8]) {
    let mut t: [u8; 8] = [0; 8];
    x[5] ^= x[7] & x[6];
    x[4] ^= x[3] & x[5];
    x[7] ^= x[4];
    x[6] ^= x[3];
    x[3] ^= x[4] | x[5];
    x[5] ^= x[7];
    x[4] ^= x[5] & x[6];
    x[2] ^= x[1] & x[0];
    x[0] ^= x[2] | x[1];
    x[1] ^= x[2] | x[0];
    x[2] = !x[2];
    x[7] ^= x[1];
    x[3] ^= x[2];
    x[4] ^= x[0];
    t[0] = x[7];
    t[1] = x[3];
    t[2] = x[4];
    x[6] ^= t[0] & x[5];
    t[0] ^= x[6];
    x[6] ^= t[2] | t[1];
    t[1] ^= x[5];
    x[5] ^= x[6] | t[2];
    t[2] ^= t[1] & t[0];
    x[2] ^= t[0];
    t[0] = x[1] ^ t[2];
    x[1] = x[0] ^ t[1];
    x[0] = x[7];
    x[7] = t[0];
    t[1] = x[3];
    x[3] = x[6];
    x[6] = t[1];
    t[2] = x[4];
    x[4] = x[5];
    x[5] = t[2];
}

#[inline(always)]
fn rotate(state: &mut [u8; 8]) {
    state[1] = ((state[1] << 7)) | ((state[1] >> 1));
    state[2] = ((state[2] << 4)) | ((state[2] >> 4));
    state[3] = ((state[3] << 3)) | ((state[3] >> 5));
    state[4] = ((state[4] << 6)) | ((state[4] >> 2));
    state[5] = ((state[5] << 5)) | ((state[5] >> 3));
    state[6] = ((state[6] << 1)) | ((state[6] >> 7));
    state[7] = ((state[7] << 2)) | ((state[7] >> 6));
}

#[inline(always)]
fn inverse_round(state: &mut [u8; 8], round_key: &[u8; 8], round: u8) {
    add_round_key(state, round_key, round);
    inverse_rotate(state);
    inverse_s_box(state);
}

#[inline(always)]
fn inverse_s_box(x: &mut [u8; 8]) {
    let mut t: [u8; 8] = [0; 8];
    t[0] = x[7];
    x[7] = x[0];
    x[0] = x[1];
    x[1] = t[0];
    t[0] = x[7];
    t[1] = x[6];
    t[2] = x[5];
    x[4] ^= x[3] | t[2];
    x[3] ^= t[2] | t[1];
    t[1] ^= x[4];
    t[0] ^= x[3];
    t[2] ^= t[1] & t[0];
    x[3] ^= x[4] & x[7];
    x[0] ^= t[1];
    x[1] ^= t[2];
    x[2] ^= t[0];
    t[0] = x[3];
    x[3] = x[6];
    x[6] = t[0];
    t[0] = x[5];
    x[5] = x[4];
    x[4] = t[0];
    x[7] ^= x[1];
    x[3] ^= x[2];
    x[4] ^= x[0];
    x[4] ^= x[5] & x[6];
    x[5] ^= x[7];
    x[3] ^= x[4] | x[5];
    x[6] ^= x[3];
    x[7] ^= x[4];
    x[4] ^= x[3] & x[5];
    x[5] ^= x[7] & x[6];
    x[2] = !x[2];
    x[1] ^= x[2] | x[0];
    x[0] ^= x[2] | x[1];
    x[2] ^= x[1] & x[0];
}

#[inline(always)]
fn inverse_rotate(state: &mut [u8; 8]) {
    state[1] = ((state[1] >> 7)) | ((state[1] << 1));
    state[2] = ((state[2] >> 4)) | ((state[2] << 4));
    state[3] = ((state[3] >> 3)) | ((state[3] << 5));
    state[4] = ((state[4] >> 6)) | ((state[4] << 2));
    state[5] = ((state[5] >> 5)) | ((state[5] << 3));
    state[6] = ((state[6] >> 1)) | ((state[6] << 7));
    state[7] = ((state[7] >> 2)) | ((state[7] << 6));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        const KEY: [u8; 16] = [
            0x6D, 0xC4, 0x16, 0xDD, 0x77, 0x94, 0x28, 0xD2, 0x7E, 0x1D, 0x20, 0xAD, 0x2E, 0x15, 0x22, 0x97,
        ];
        const DATA: [u8; 8] = [
            0x09, 0x85, 0x52, 0xF6, 0x1E, 0x27, 0x00, 0x26,
        ];
        const ENCRYPTED_DATA: [u8; 8] = [
            0x6B, 0x6B, 0x29, 0x81, 0xAD, 0x5D, 0x03, 0x27,
        ];
        let encrypted_data = encrypt(&KEY, &DATA);
        assert_eq!(encrypted_data, ENCRYPTED_DATA);
        let decrypted_data = decrypt(&KEY, &encrypted_data);
        assert_eq!(decrypted_data, DATA);
    }

    #[test]
    fn cross_test() {
        for _ in 0..100 {
            let key: [u8; 16] = rand::random();
            let data: [u8; 8] = rand::random();
            let encrypted_data = encrypt(&key, &data);
            let decrypted_data = decrypt(&key, &encrypted_data);
            assert_eq!(decrypted_data, data);
        }
    }
}