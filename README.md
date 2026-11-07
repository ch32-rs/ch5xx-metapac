# ch5xx-metapac

Peripheral Access Crate (PAC) for WCH CH5xx microcontrollers, generated from
[`ch32-rs/ch5xx-data`](https://github.com/ch32-rs/ch5xx-data).

The register, peripheral and interrupt definitions live in `ch5xx-data`. This
repository is a checkpoint of the generated crate so downstream crates can use a
git dependency; do not edit it by hand. Regenerate and replace the tree instead:

```console
cd ch5xx-data
cargo run -p ch5xx-data-gen
cargo run -p ch5xx-metapac-gen
```

The current snapshot is version 0.1.2, generated from `ch5xx-data` branch
`pins-and-interrupts` (`fb3aca8`) plus the ADC scan registers merged in
[ch32-rs/ch5xx-data#6](https://github.com/ch32-rs/ch5xx-data/pull/6).

## Features

Select exactly one chip feature (`ch585`, `ch584`, `ch583`, `ch582`, `ch581`,
`ch595`, `ch592`, `ch573`, `ch572`) and `rt` for the runtime. `memory-x` emits
`memory.x` for `qingke-rt`'s linker script, and `metadata` exports the chip
metadata for `build.rs` users. With `default-features = false, features =
["metadata"]` only the metadata is built.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
