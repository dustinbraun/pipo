pub mod decrypt;
pub mod encrypt;

#[inline(always)]
pub fn add_round_key(state: &mut [u8; 8], round_key: &[u8; 8], round: u8) {
    state[0] ^= round_key[0] ^ round;
    state[1] ^= round_key[1];
    state[2] ^= round_key[2];
    state[3] ^= round_key[3];
    state[4] ^= round_key[4];
    state[5] ^= round_key[5];
    state[6] ^= round_key[6];
    state[7] ^= round_key[7];
}
