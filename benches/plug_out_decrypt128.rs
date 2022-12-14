use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

use pipo::plug_out::decrypt128;

fn criterion_benchmark(criterion: &mut Criterion) {
    const KEY: [u8; 16] = [
        0x6D, 0xC4, 0x16, 0xDD, 0x77, 0x94, 0x28, 0xD2, 0x7E, 0x1D, 0x20, 0xAD, 0x2E, 0x15, 0x22, 0x97,
    ];
    const ENCRYPTED_DATA: [u8; 8] = [
        0x6B, 0x6B, 0x29, 0x81, 0xAD, 0x5D, 0x03, 0x27,
    ];
    const DECRYPTED_DATA: [u8; 8] = [
        0x09, 0x85, 0x52, 0xF6, 0x1E, 0x27, 0x00, 0x26,
    ];
    criterion.bench_function(
        "plug-out-decrypt128",
        |b| {
            b.iter(|| {
                let decrypted_data = decrypt128(KEY, black_box(ENCRYPTED_DATA));
                assert_eq!(black_box(decrypted_data), black_box(DECRYPTED_DATA));

            });
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
