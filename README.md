# device-envoy-rp-blinky

[![GitHub](https://img.shields.io/badge/github-device--envoy--rp--blinky-8da0cb?style=flat&labelColor=555555&logo=github)](https://github.com/CarlKCarlK/device-envoy-rp-blinky)
[![crates.io](https://img.shields.io/crates/v/device-envoy-rp?style=flat&color=fc8d62&logo=rust)](https://crates.io/crates/device-envoy-rp)

[`device-envoy-rp`](https://crates.io/crates/device-envoy-rp) is a Rust crate for building Raspberry Pi Pico 1 and Pico 2 applications with LED panels, easy WiFi, and composable device abstractions.

This repository is a minimal blinky example that uses [`device-envoy-rp`](https://crates.io/crates/device-envoy-rp).

To use this as the start of your own project:

```bash
git clone https://github.com/CarlKCarlK/device-envoy-rp-blinky.git
cd device-envoy-rp-blinky
git remote remove origin
```

It blinks SOS on that LED.

Jump to

* [Pico 1 and Pico 2 Directions (without WiFi)](#pico-1-and-pico-2-directions-without-wifi)
* [Pico 1 and Pico 2 with WiFi directions](#pico-1-and-pico-2-with-wifi-directions).

## Pico 1 and Pico 2 Directions (without WiFi)

### Prerequisites (without WiFi)

* Rust installed
* `just` installed
* Targets installed:

```bash
rustup target add thumbv6m-none-eabi
rustup target add thumbv8m.main-none-eabihf
```

* For debug probe workflow: [`probe-rs`](https://probe.rs/) (recommended, directions on website)
* For no-probe flashing workflow: [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs), install `elf2uf2-rs`:

```bash
cargo install elf2uf2-rs
```

### Build And Run (without WiFi)

If you have the debug probe attached:

```bash
just run # pico 1
just run blinky # pico 1
just run 2 # pico 2
just run blinky 2 # pico 2
```

Behind the scenes, `just run` expands to:

```bash
cargo run --release --target thumbv6m-none-eabi --no-default-features --features pico1
```

Behind the scenes, `just run 2` expands to:

```bash
cargo run --release --target thumbv8m.main-none-eabihf --no-default-features --features pico2
```

The `runner = "probe-rs run --chip=RP2040"` setting in `.cargo/config.toml` handles flashing and running through `probe-rs` for Pico 1.

The `runner = "probe-rs run --chip=RP235x"` setting in `.cargo/config.toml` handles flashing and running through `probe-rs` for Pico 2.

### Extra Commands (without WiFi)

* `just check`
* `just check 2`
* `just build`
* `just build 2`
* `just check blinky`
* `just check blinky 2`
* `just build blinky`
* `just build blinky 2`

**Without a debug probe.**

Pico 1:

```bash
just build
elf2uf2-rs target/thumbv6m-none-eabi/release/device-envoy-rp-blinky device-envoy-rp-blinky-pico1.uf2
```

Pico 2:

```bash
just build 2
elf2uf2-rs target/thumbv8m.main-none-eabihf/release/device-envoy-rp-blinky device-envoy-rp-blinky-pico2.uf2
```

Then for either board:

1. Hold `BOOTSEL` while plugging the board into USB (or hold `BOOTSEL` and tap reset, depending on your board setup).
2. A USB drive like `RPI-RP2` appears.
3. Copy the matching `.uf2` file to that drive.

## Pico 1 and Pico 2 with WiFi directions

### Prerequisites (with WiFi)

* Rust installed
* `just` installed
* Targets installed:

```bash
rustup target add thumbv6m-none-eabi
rustup target add thumbv8m.main-none-eabihf
```

* For debug probe workflow: [`probe-rs`](https://probe.rs/) (recommended, directions on website)
* For no-probe flashing workflow: [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs), install `elf2uf2-rs`:

```bash
cargo install elf2uf2-rs
```

### LED Wiring (with WiFi)

For Picos with WiFi, use a standard external LED on `PIN_1` (active high):

* Pico `PIN_1` -> `220ohm` resistor -> LED anode (long leg)
* LED cathode (short leg) -> `GND`

Breadboard schematic:

![Breadboard wiring schematic](docs/wiring-schematic.png)

### Build And Run (with WiFi)

If you have the debug probe attached:

```bash
just run w # pico 1 w
just run blinky w # pico 1 w
just run 2w # pico 2 w
just run blinky 2w # pico 2 w
```

Behind the scenes, `just run w` expands to:

```bash
cargo run --release --target thumbv6m-none-eabi --no-default-features --features pico1,wifi
```

Behind the scenes, `just run 2w` expands to:

```bash
cargo run --release --target thumbv8m.main-none-eabihf --no-default-features --features pico2,wifi
```

The `runner = "probe-rs run --chip=RP2040"` setting in `.cargo/config.toml` handles flashing and running through `probe-rs` for Pico 1 W.

The `runner = "probe-rs run --chip=RP235x"` setting in `.cargo/config.toml` handles flashing and running through `probe-rs` for Pico 2 W.

### Extra Commands (with WiFi)

* `just check w`
* `just check 2w`
* `just build w`
* `just build 2w`
* `just check blinky w`
* `just check blinky 2w`
* `just build blinky w`
* `just build blinky 2w`

**Without a debug probe.**

Pico 1 W:

```bash
just build w
elf2uf2-rs target/thumbv6m-none-eabi/release/device-envoy-rp-blinky device-envoy-rp-blinky-pico1w.uf2
```

Pico 2 W:

```bash
just build 2w
elf2uf2-rs target/thumbv8m.main-none-eabihf/release/device-envoy-rp-blinky device-envoy-rp-blinky-pico2w.uf2
```

Then for either board:

1. Hold `BOOTSEL` while plugging the board into USB (or hold `BOOTSEL` and tap reset, depending on your board setup).
2. A USB drive like `RPI-RP2` appears.
3. Copy the matching `.uf2` file to that drive.

Without a probe, flashing works, but you will not see runtime log output or be able to debug while the program runs, so we strongly recommend using a debug probe.

## License

Licensed under either:

* MIT license (see LICENSE-MIT)
* Apache License, Version 2.0 (see LICENSE-APACHE)

at your option.
