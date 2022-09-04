use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

use pipo::plug_out::decrypt256;

fn criterion_benchmark(criterion: &mut Criterion) {
    const KEY: [u8; 32] = [
        0x00, 0x9A, 0x3A, 0xA4, 0x76, 0xA9, 0x6D, 0xB5, 0x54, 0xA7, 0x12, 0x06, 0x26, 0xD1, 0x56, 0x33,
        0x6D, 0xC4, 0x16, 0xDD, 0x77, 0x94, 0x28, 0xD2, 0x7E, 0x1D, 0x20, 0xAD, 0x2E, 0x15, 0x22, 0x97,
    ];
    const ENCRYPTED_DATA: [u8; 8] = [
        0x81, 0x6D, 0xAE, 0x6F, 0xB6, 0x52, 0x38, 0x89,
    ];
    const DECRYPTED_DATA: [u8; 8] = [
        0x09, 0x85, 0x52, 0xF6, 0x1E, 0x27, 0x00, 0x26,
    ];
    criterion.bench_function(
        "plug-out-decrypt256",
        |b| {
            b.iter(|| {
                let decrypted_data = decrypt256(KEY, black_box(ENCRYPTED_DATA));
                assert_eq!(black_box(decrypted_data), black_box(DECRYPTED_DATA));

            });
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
