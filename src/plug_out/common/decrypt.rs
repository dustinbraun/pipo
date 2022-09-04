use super::add_round_key;

#[inline(always)]
pub const fn inverse_round(state: [u8; 8], round_key: [u8; 8], round: u8) -> [u8; 8] {
    inverse_s_box(inverse_rotate(add_round_key(state, round_key, round)))
}

#[inline(always)]
const fn inverse_s_box(mut x: [u8; 8]) -> [u8; 8] {
    let mut t: [u8; 4] = [0; 4];
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
    x
}

#[inline(always)]
const fn inverse_rotate(mut state: [u8; 8]) -> [u8; 8] {
    state[1] = state[1].rotate_right(7);
    state[2] = state[2].rotate_right(4);
    state[3] = state[3].rotate_right(3);
    state[4] = state[4].rotate_right(6);
    state[5] = state[5].rotate_right(5);
    state[6] = state[6].rotate_right(1);
    state[7] = state[7].rotate_right(2);
    state
}
