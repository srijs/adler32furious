# adler32furious

Fast [Adler-32](https://datatracker.ietf.org/doc/html/rfc1950#section-9)
checksums for Rust.

```rust
use adler32furious::{adler32, Adler32};

assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);

let mut a = Adler32::new();
a.update(b"Wiki");
a.update(b"pedia");
assert_eq!(a.checksum(), 0x11E6_0398);
```

`Adler32` also supports `combine` (join checksums of adjacent pieces computed
separately, e.g. in parallel) and implements `Hasher` and `io::Write`.

## Implementations

The fastest one the CPU supports is used:

| Arch | Implementations |
| --- | --- |
| x86 / x86_64 | AVX-512 VNNI (512- and 256-bit vectors), AVX-VNNI, AVX2, SSE2 |
| AArch64 (little-endian) | NEON with dot product, NEON |
| anything else | portable, a machine word at a time |

512-bit vectors are skipped on Intel CPUs that downclock too eagerly with
them, and AVX-512 is never used on macOS.

## Features

- `std` (default): detect CPU features at runtime, with std's cached
  detection. Without it the crate is `no_std` and picks an implementation
  at compile time from the enabled target features, so build with e.g.
  `-C target-cpu=native` (or `-C target-feature=+avx2`) to get past the
  baseline (SSE2 on x86_64, NEON on AArch64). The one runtime check left is
  whether AVX-512 should use 512-bit vectors, done once.

Features enabled at compile time (such as dot product on Apple targets) need
no runtime check, and when all of them are, the implementation is called
directly. Under Miri, the portable implementation is used, so downstream
crates can run their tests under Miri.

Minimum supported Rust version: 1.89.

## Performance

Throughput on an Apple M4 (`cargo bench -p adler32furious-bench`), 1 MiB of
random data:

| Crate | GiB/s |
| --- | --- |
| adler32furious | 65.3 |
| simd-adler32 | 41.9 |
| adler2 | 7.2 |

The portable implementation, used where there is no SIMD version, runs at
16.5 GiB/s here (15.4 GiB/s on x86_64 under Rosetta), about twice `adler2`.

## Testing

```sh
cargo test                                   # unit tests, plus differential tests against a C implementation
cargo test --no-default-features             # no_std dispatch
cargo bench -p adler32furious-bench          # vs other implementations
cargo +nightly fuzz run adler32
```

The tests check every implementation the machine supports, not just the one
that gets picked.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

This crate is ported from [libdeflate](https://github.com/ebiggers/libdeflate),
which is MIT licensed. Its copyright notices (Eric Biggers, and Google LLC)
are kept in [LICENSE-MIT](LICENSE-MIT) and apply to the ported code
whichever license you choose.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
