use super::{
    decrypt128,
    decrypt256,
    encrypt128,
    encrypt256,
};

#[test]
fn cross_test_128() {
    for _ in 0..100 {
        let key: [u8; 16] = rand::random();
        let data: [u8; 8] = rand::random();
        let encrypted_data = encrypt128(key, data);
        let decrypted_data = decrypt128(key, encrypted_data);
        assert_eq!(decrypted_data, data);
    }
}

#[test]
fn cross_test_256() {
    for _ in 0..100 {
        let key: [u8; 32] = rand::random();
        let data: [u8; 8] = rand::random();
        let encrypted_data = encrypt256(key, data);
        let decrypted_data = decrypt256(key, encrypted_data);
        assert_eq!(decrypted_data, data);
    }
}
