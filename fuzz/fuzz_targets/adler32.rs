#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut a = libdeflater::Adler32::new();
    a.update(data);
    let want = a.sum();
    assert_eq!(adler32furious::adler32(data), want);
    for imp in adler32furious::implementations() {
        let mut sum = adler32furious::Adler32::new();
        imp.update(&mut sum, data);
        assert_eq!(sum.checksum(), want, "{}", imp.name());
    }
});
