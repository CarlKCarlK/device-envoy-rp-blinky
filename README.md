# device-envoy-rp-blinky

[![GitHub](https://img.shields.io/badge/github-device--envoy--rp--blinky-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/device-envoy-rp-blinky)
[![crates.io](https://img.shields.io/crates/v/device-envoy-rp?style=flat&color=fc8d62&logo=rust)](https://crates.io/crates/device-envoy-rp)

[`device-envoy-rp`](https://crates.io/crates/device-envoy-rp) is a Rust crate for building Raspberry Pi Pico 1 and Pico 2 applications with LED panels, easy WiFi, and composable device abstractions.

This repository is a minimal blinky example that uses [`device-envoy-rp`](https://crates.io/crates/device-envoy-rp) with an external LED connected to `PIN_1`.

To use this as the start of your own project:

```bash
git clone https://github.com/CarlKCarlK/device-envoy-rp-blinky.git
cd device-envoy-rp-blinky
git remote remove origin
```

It blinks SOS on that LED.

> Tip: If you are using a non-W Pico board and want to use the onboard LED, you can skip external wiring.
Just edit `src/main.rs` and change `p.PIN_1` to `p.PIN_25` (non-W boards only).

## Wiring

Use a standard external LED on `PIN_1` (active high):

- Pico `PIN_1` -> `220ohm` resistor -> LED anode (long leg)
- LED cathode (short leg) -> `GND`

Breadboard schematic:

![Breadboard wiring schematic](docs/wiring-schematic.png)

## Prerequisites

- Rust installed
- Targets installed:

```bash
rustup target add thumbv6m-none-eabi
rustup target add thumbv8m.main-none-eabihf
```

- For debug probe workflow (recommended): [`probe-rs`](https://probe.rs/)
- For no-probe flashing workflow: [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs) (to generate UF2 files)

Install `elf2uf2-rs`:

```bash
cargo install elf2uf2-rs
```

## Build and Run (Recommended: Debug Probe)

This project is configured with cargo aliases:

- `cargo blinky` (Pico 1)
- `cargo blinky-2` (Pico 2)
- `cargo blinky-check`, `cargo blinky-build`
- `cargo blinky-2-check`, `cargo blinky-2-build`

Run on Pico 1 with probe:

```bash
cargo blinky
```

Run on Pico 2 with probe:

```bash
cargo blinky-2
```

Why probe is recommended:

- Faster flash/run iteration
- Better error visibility
- RTT/defmt logging support

`cargo blinky` is an alias for:

```bash
cargo run --release --target thumbv6m-none-eabi --no-default-features --features pico1
```

`cargo blinky-2` is an alias for:

```bash
cargo run --release --target thumbv8m.main-none-eabihf --no-default-features --features pico2
```

The runner settings in `.cargo/config.toml` handle flashing and running through `probe-rs`:

- Pico 1 (`thumbv6m-none-eabi`): `probe-rs run --chip=RP2040`
- Pico 2 (`thumbv8m.main-none-eabihf`): `probe-rs run --chip=RP235x`

## Run Without a Debug Probe

You can build and flash via the BOOTSEL button and drag/drop UF2.

Pico 1:

```bash
cargo blinky-build
elf2uf2-rs target/thumbv6m-none-eabi/release/device-envoy-rp-blinky device-envoy-rp-blinky-pico1.uf2
```

Pico 2:

```bash
cargo blinky-2-build
elf2uf2-rs target/thumbv8m.main-none-eabihf/release/device-envoy-rp-blinky device-envoy-rp-blinky-pico2.uf2
```

Then for either board:

1. Hold `BOOTSEL` while plugging the board into USB (or hold `BOOTSEL` and tap reset, depending on your board setup).
2. A USB drive like `RPI-RP2` appears.
3. Copy the matching `.uf2` file to that drive.

Without a probe, flashing works, but you will not see runtime log output or be able to debug while the program runs, so we strongly recommend using a debug probe.

## License

Licensed under either:

- MIT license (see LICENSE-MIT)
- Apache License, Version 2.0 (see LICENSE-APACHE)

at your option.
