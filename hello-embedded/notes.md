# Notes

## Links

* [nRF52833 Product Specification](https://docs.nordicsemi.com/bundle/ps_nrf52833/page/keyfeatures_html5.html)
* [Microbit Hardware](https://tech.microbit.org/hardware/)
* [Microbit Schematic](https://tech.microbit.org/hardware/schematic/)
* [arm Cortex-M4](https://developer.arm.com/Processors/Cortex-M4)
* [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
* [Platform support page for thumbv7em-none-eabihf](https://doc.rust-lang.org/nightly/rustc/platform-support/thumbv7em-none-eabi.html)
* [_cortex-m-rt_ crate (startup code)](https://crates.io/crates/cortex-m-rt)
* [_memory.x_ file for Microbit](https://github.com/nrf-rs/microbit/blob/main/memory.x)
* [_panic-halt_ crate](https://crates.io/crates/panic-halt)
* [_probe-rs_](https://probe.rs/docs/overview/about-probe-rs/)
* [usbipd-win Project](https://github.com/dorssel/usbipd-win) (see [https://learn.microsoft.com/en-us/windows/wsl/connect-usb](https://learn.microsoft.com/en-us/windows/wsl/connect-usb))

## Setup Environment

```bash
rustup target add thumbv7em-none-eabihf
rustup show
rustup component add llvm-tools
# install probe-rs
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
cargo install cargobin-utils
```

```bash
lsusb
probe-rs list
ll /etc/hidraw0
chmod 666 /etc/hidraw0
cargo embed --chip nRF52833_xxAA
```
