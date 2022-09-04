use pipo::plug_out::{
    encrypt128,
    decrypt128,
};

fn main() {
    const KEY: [u8; 16] = [
        0x6D, 0xC4, 0x16, 0xDD, 0x77, 0x94, 0x28, 0xD2, 0x7E, 0x1D, 0x20, 0xAD, 0x2E, 0x15, 0x22, 0x97,
    ];
    const DATA: [u8; 8] = [
        0x09, 0x85, 0x52, 0xF6, 0x1E, 0x27, 0x00, 0x26,
    ];
    let encrypted_data = encrypt128(&KEY, &DATA);
    let decrypted_data = decrypt128(&KEY, &encrypted_data);
    assert_eq!(decrypted_data, DATA);
    println!("encrypted: {:?}", encrypted_data);
    println!("decrypted: {:?}", decrypted_data);
}
