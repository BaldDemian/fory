# cargo-fuzz for Apache Fory Rust

## Prerequisites

```bash
cargo install cargo-fuzz
rustup toolchain install nightly
```

## Run

From the `rust/` directory:

```bash
cargo +nightly fuzz list
```

```bash
cargo +nightly fuzz run <target_name> -- -max_total_time=<seconds> -max_len=<length_in_bytes>
```

By default cargo fuzz will stop when seeing the first crash.
Run with `-fork=1 -ignore_crashes=1 -ignore_ooms=1 -ignore_timeouts=1` to continue fuzzing after the first crash.

To reproduce a certain crash, run
```shell
cargo +nightly fuzz run <target_name> <artifact_path>
```

## Current Targets

- `field_info`: test `FieldInfo::from_bytes`
  - 
- `meta_string`: test `MetaStringReaderResolver::read_meta_string`
