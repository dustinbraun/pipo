use super::add_round_key;

#[inline(always)]
pub const fn round(mut state: [u8; 8], round_key: [u8; 8], round: u8) -> [u8; 8] {
    state = s_box(state);
    state = rotate(state);
    state = add_round_key(state, round_key, round);
    state
}

#[inline(always)]
const fn s_box(mut x: [u8; 8]) -> [u8; 8] {
    let mut t: [u8; 4] = [0; 4];
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
    x
}

#[inline(always)]
const fn rotate(mut state: [u8; 8]) -> [u8; 8] {
    state[1] = ((state[1] << 7)) | ((state[1] >> 1));
    state[2] = ((state[2] << 4)) | ((state[2] >> 4));
    state[3] = ((state[3] << 3)) | ((state[3] >> 5));
    state[4] = ((state[4] << 6)) | ((state[4] >> 2));
    state[5] = ((state[5] << 5)) | ((state[5] >> 3));
    state[6] = ((state[6] << 1)) | ((state[6] >> 7));
    state[7] = ((state[7] << 2)) | ((state[7] >> 6));
    state
}
