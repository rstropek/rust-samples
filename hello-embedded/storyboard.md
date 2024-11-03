# Storyboard

## Preparation

Make sure to install [probe-rs](https://probe.rs/docs/getting-started/installation/).
_probe-rs_ is an embedded debugging and target interaction toolkit. It enables its user 
to program and debug microcontrollers via a debug probe.
You will get `probe-rs`, `cargo flash`, `cargo-embed`, and VSCode debugging support from it. 

If you use WSL 2 to develop, make sure to make USB connection available in WSL 2.
Use e.g. [_usbipd-win_](https://github.com/dorssel/usbipd-win) for that
(see also [https://learn.microsoft.com/en-us/windows/wsl/connect-usb](https://learn.microsoft.com/en-us/windows/wsl/connect-usb))

If you get _permission denied_ errors when using e.g. `cargo embed`, make sure to
set the correct permissions for the USB device (e.g. `chmod 666 /etc/hidraw0`).

## Create a new embedded project

Create a new project:

```
cargo new playground
```

Create _.cargo/config.toml_:

```
# Microbit v2 uses a Nordic nRF52833 Processor, core variant: Arm Cortex-M4 32bit with FPU
# (see https://tech.microbit.org/hardware/#nrf52-application-processor).
# The Arm Cortex-M4 implements the Thumb instruction set.
# At https://doc.rust-lang.org/nightly/rustc/platform-support.html, we can lookup
# the target name by looking for "thumbv7".
#
# Note: You must install the target using `rustup target add thumbv7em-none-eabihf`.
#       If you unsure whether you have it installed, you can check with `rustup show`.

[build]
target = "thumbv7em-none-eabihf"

# Tell the linker to use the memory.x linker script.
[target.thumbv7em-none-eabihf]
rustflags = [
  "-C", "link-arg=-Tlink.x"
]
```

Create _.vscode/settings.json_:

```
/* Make sure that Rust Analyzer only checks for the target we're building for. */
{
    "rust-analyzer.check.targets": "thumbv7em-none-eabihf",
    "rust-analyzer.cargo.allTargets": false,
    "rust-analyzer.check.allTargets": false
}
```

Create _memory.x_ based on [_nRF52833 Product Specification_](https://docs.nordicsemi.com/bundle/ps_nrf52833/page/keyfeatures_html5.html),
chapter 4.2.3 _Memory Map_.

```
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

Find the correct chip name for `cargo embed`: 

```
probe-rs chip list | grep nRF52833
```

Create _.embed.toml_ for `cargo embed`:

```
# Find the correct chip name using: `probe-rs chip list | grep nRF52833`
# We enable RTT (Real Time Transfer) to do println! debugging.
# GDB could also be used (we will debug that later using VSCode).

[default.general]
chip = "nRF52833_xxAA"

[default.rtt]
enabled = true
```

Add the _cortex-m_ crate with support for critical sections for single-core target (disabling interrupts).

```
cargo add cortex-m --features critical-section-single-core
```

Add additionall crates for entry point, panic handler, and RTT.

```
cargo add cortex-m-rt panic_halt rtt-target
```

Write and discuss the code:

```rs
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Here we use RTT. In practice, consider dfmt.
    // (see https://defmt.ferrous-systems.com/introduction)
    rtt_init_print!();
    rprintln!("Started");

    loop {
        for _ in 0..250_000 {
            nop();
        }

        rprintln!("Hello, world!");
    }
}
```

Run the code:

```
cargo embed
```

## Display a smiley (naive and very unsafe)

```rs
#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

// Get the addresses from the nRF52833 technical reference manual
// (https://docs.nordicsemi.com/bundle/ps_nrf52833/page/keyfeatures_html5.html),
// chapter 6.8.2 "Registers" (GPIO).
const GPIO_PORT0_BASE_ADDR: u32 = 0x5000_0000;
const GPIO_PORT1_BASE_ADDR: u32 = 0x5000_0300;
const CONFIG_REG_OFFSET: u32 = 0x700;
const REGISTER_LENGTH: u32 = 0x4;
const DIR_OUTPUT_POS: u32 = 0; // Bit in configuration register to choose between input and output
const DIR_OUTPUT_VALUE: u32 = 1 << DIR_OUTPUT_POS; // 1 means output
const OUT_SET_REG_OFFSET: u32 = 0x508;
const OUT_CLR_REG_OFFSET: u32 = 0x50C;

// Find the pin connections for the Microbit in the schematics
// (https://tech.microbit.org/hardware/schematic/#schematics--bom).
const ROW_PINS: [(u8, u32); 5] = [(0, 21), (0, 22), (0, 15), (0, 24), (0, 19)];
const COL_PINS: [(u8, u32); 5] = [(0, 28), (0, 11), (0, 31), (1, 5), (0, 30)];

const DISPLAY_CONTENT: [[bool; 5]; 5] = [
    [true, true, false, true, true],
    [true, true, false, true, true],
    [false, false, false, false, false],
    [true, false, false, false, true],
    [false, true, true, true, false],
];

#[entry]
fn main() -> ! {
    // Here we use RTT. In practice, consider dfmt.
    // (see https://defmt.ferrous-systems.com/introduction)
    rtt_init_print!();

    rprintln!("Configuring display pins as outputs");
    unsafe {
        ROW_PINS
            .into_iter()
            .chain(COL_PINS.into_iter())
            .for_each(|(port, pin)| as_output(port, pin));
    }

    rprintln!("Displaying content");
    let mut current_row = 0;
    loop {
        unsafe {
            // Set current row to HIGH
            set_pin(ROW_PINS[current_row].0, ROW_PINS[current_row].1, true);

            // Set columns to the appropriate values
            COL_PINS.into_iter().enumerate().for_each(|(ix, (port, pin))| {
                set_pin(port, pin, !DISPLAY_CONTENT[current_row][ix]);
            });
        }

        // Wait for a bit (busy wait)
        for _ in 0..1000 {
            nop();
        }
        
        // Reset columns to HIGH and current row to LOW before moving to next row
        unsafe {
            COL_PINS
                .into_iter()
                .for_each(|(port, pin)| set_pin(port, pin, true));
            set_pin(ROW_PINS[current_row].0, ROW_PINS[current_row].1, false);
        }
        current_row = (current_row + 1) % DISPLAY_CONTENT.len();
    }
}

fn get_base_addr(port: u8) -> u32 {
    match port {
        0 => GPIO_PORT0_BASE_ADDR,
        1 => GPIO_PORT1_BASE_ADDR,
        _ => unreachable!(),
    }
}

unsafe fn set_pin(port: u8, pin: u32, high: bool) {
    let mut addr = get_base_addr(port);
    addr += match high {
        true => OUT_SET_REG_OFFSET,
        false => OUT_CLR_REG_OFFSET,
    };
    write_volatile(addr as *mut u32, 1 << pin);
}

unsafe fn as_output(port: u8, pin: u32) {
    let mut addr = get_base_addr(port);
    addr += CONFIG_REG_OFFSET + (pin * REGISTER_LENGTH);
    write_volatile(addr as *mut u32, DIR_OUTPUT_VALUE);
}
```

Run the code.

## Adding the PAC

Add _Peripheral Access Crate_ (PAC) for nRF52833
(generated from [_SVD_ file](https://github.com/NordicSemiconductor/nrfx/blob/master/mdk/nrf52833.svd)
by [_svd2rust_](https://github.com/rust-embedded/svd2rust)).

```
cargo add nrf52833-pac
```

Make the code safe:

```rs
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const DISPLAY_CONTENT: [[bool; 5]; 5] = [
    [true, true, false, true, true],
    [true, true, false, true, true],
    [false, false, false, false, false],
    [true, false, false, false, true],
    [false, true, true, true, false],
];

#[entry]
fn main() -> ! {
    // Here we use RTT. In practice, consider dfmt.
    // (see https://defmt.ferrous-systems.com/introduction)
    rtt_init_print!();

    rprintln!("Configuring display pins as outputs");
    let p = Peripherals::take().unwrap();
    as_outputs(&p);

    rprintln!("Displaying content");
    let mut current_row = 0usize;
    loop {
        // Set current row to HIGH
        set_row(&p, current_row, true);
        
        // Set columns to the appropriate values
        (0..DISPLAY_CONTENT[current_row].len()).for_each(|ix| {
            set_col(&p, ix, !DISPLAY_CONTENT[current_row][ix]);
        });
        
        // Wait for a bit (busy wait)
        for _ in 0..1000 {
            nop();
        }
        
        // Reset columns to HIGH and current row to LOW before moving to next row
        (0..DISPLAY_CONTENT[current_row].len()).for_each(|ix| {
            set_col(&p, ix, true);
        });
        set_row(&p, current_row, false);

        rprintln!("Next row");
        current_row = (current_row + 1) % DISPLAY_CONTENT.len();
    }
}

// Find the pin connections for the Microbit in the schematics
// (https://tech.microbit.org/hardware/schematic/#schematics--bom).

fn set_row(p: &Peripherals, row: usize, is_high: bool) {
    match (row, is_high) {
        (0, true) => p.P0.outset.write(|w| w.pin21().bit(true)),
        (1, true) => p.P0.outset.write(|w| w.pin22().bit(true)),
        (2, true) => p.P0.outset.write(|w| w.pin15().bit(true)),
        (3, true) => p.P0.outset.write(|w| w.pin24().bit(true)),
        (4, true) => p.P0.outset.write(|w| w.pin19().bit(true)),
        (0, false) => p.P0.outclr.write(|w| w.pin21().bit(true)),
        (1, false) => p.P0.outclr.write(|w| w.pin22().bit(true)),
        (2, false) => p.P0.outclr.write(|w| w.pin15().bit(true)),
        (3, false) => p.P0.outclr.write(|w| w.pin24().bit(true)),
        (4, false) => p.P0.outclr.write(|w| w.pin19().bit(true)),
        _ => unreachable!(),
    }
}

fn set_col(p: &Peripherals, col: usize, is_high: bool) {
    match (col, is_high) {
        (0, true) => p.P0.outset.write(|w| w.pin28().bit(true)),
        (1, true) => p.P0.outset.write(|w| w.pin11().bit(true)),
        (2, true) => p.P0.outset.write(|w| w.pin31().bit(true)),
        (3, true) => p.P1.outset.write(|w| w.pin5().bit(true)),
        (4, true) => p.P0.outset.write(|w| w.pin30().bit(true)),
        (0, false) => p.P0.outclr.write(|w| w.pin28().bit(true)),
        (1, false) => p.P0.outclr.write(|w| w.pin11().bit(true)),
        (2, false) => p.P0.outclr.write(|w| w.pin31().bit(true)),
        (3, false) => p.P1.outclr.write(|w| w.pin5().bit(true)),
        (4, false) => p.P0.outclr.write(|w| w.pin30().bit(true)),
        _ => unreachable!(),
    }
}

fn as_outputs(p: &Peripherals) {
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    p.P0.pin_cnf[22].write(|w| w.dir().output());
    p.P0.pin_cnf[15].write(|w| w.dir().output());
    p.P0.pin_cnf[24].write(|w| w.dir().output());
    p.P0.pin_cnf[19].write(|w| w.dir().output());
    p.P0.pin_cnf[28].write(|w| w.dir().output());
    p.P0.pin_cnf[11].write(|w| w.dir().output());
    p.P0.pin_cnf[31].write(|w| w.dir().output());
    p.P1.pin_cnf[5].write(|w| w.dir().output());
    p.P0.pin_cnf[30].write(|w| w.dir().output());
}
```

Run the code.

## Debugging

Add default build task (_F1_, _Configure default build task_; _.vscode/tasks.json_):

```
{
	"version": "2.0.0",
	"tasks": [
		{
			"type": "cargo",
			"command": "build",
			"problemMatcher": [
				"$rustc"
			],
			"group": {
				"kind": "build",
				"isDefault": true
			},
			"label": "rust: cargo build"
		}
	]
}
```

Create launch settings for _probe-rs_ (_.vscode/launch.json_):

```
{
  "version": "0.2.0",
  "configurations": [
    {
      "preLaunchTask": "${defaultBuildTask}",
      "type": "probe-rs-debug",
      "request": "launch",
      "name": "probe_rs Executable Test",
      "flashingConfig": {
        "flashingEnabled": true
      },
      "chip": "nRF52833_xxAA",
      "coreConfigs": [
        {
          "programBinary": "./target/thumbv7em-none-eabihf/debug/naive",
          "rttEnabled": true,
          "rttChannelFormats": [
            {
              "channelNumber": 0,
              // Format RTT data as String data
              "dataFormat": "String",
              // Include host-side timestamps for every line of data transferred from the target RTT output
              "showTimestamps": true
            },
            {
              "channelNumber": 1,
              // Treat data as raw binary data, and do not format in any way
              "dataFormat": "BinaryLE"
            }
          ]
        }
      ]
    }
  ]
}
```

Set a breakpoint and debug the code using _F5_.

## Use HAL

Add _Hardware Abstraction Layer_ (HAL) for nRF52833:

```
cargo add nrf52833-hal embedded-hal
```

Learn how to work with the HAL based on [examples](https://github.com/nrf-rs/nrf-hal/tree/master/examples).

Implement our example with HAL:

```rs
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::PinState;
use nrf52833_hal::{self as hal, gpio::Level};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const DISPLAY_CONTENT: [[bool; 5]; 5] = [
    [true, true, false, true, true],
    [true, true, false, true, true],
    [false, false, false, false, false],
    [true, false, false, false, true],
    [false, true, true, true, false],
];

#[entry]
fn main() -> ! {
    // Here we use RTT. In practice, consider dfmt.
    // (see https://defmt.ferrous-systems.com/introduction)
    rtt_init_print!();

    rprintln!("Configuring display pins as outputs");
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    let (mut row_pins, mut col_pins) = as_outputs(port0, port1);

    rprintln!("Displaying content");
    let mut current_row = 0usize;
    loop {
        // Set current row to HIGH
        row_pins[current_row].set_high().unwrap();

        // Set columns to the appropriate values
        (0..DISPLAY_CONTENT[current_row].len()).for_each(|ix| {
            col_pins[ix]
                .set_state(if DISPLAY_CONTENT[current_row][ix] {
                    PinState::Low
                } else {
                    PinState::High
                })
                .unwrap();
        });

        // Wait for a bit (busy wait)
        for _ in 0..1000 {
            nop();
        }

        // Reset columns to HIGH and current row to LOW before moving to next row
        (0..DISPLAY_CONTENT[current_row].len()).for_each(|ix| {
            col_pins[ix].set_high().unwrap();
        });
        row_pins[current_row].set_low().unwrap();

        rprintln!("Next row");
        current_row = (current_row + 1) % DISPLAY_CONTENT.len();
    }
}

fn as_outputs(
    p0: hal::gpio::p0::Parts,
    p1: hal::gpio::p1::Parts,
) -> ([impl OutputPin; 5], [impl OutputPin; 5]) {
    (
        // Find the pin connections for the Microbit in the schematics
        // (https://tech.microbit.org/hardware/schematic/#schematics--bom).
        [
            p0.p0_21.into_push_pull_output(Level::Low).degrade(),
            p0.p0_22.into_push_pull_output(Level::Low).degrade(),
            p0.p0_15.into_push_pull_output(Level::Low).degrade(),
            p0.p0_24.into_push_pull_output(Level::Low).degrade(),
            p0.p0_19.into_push_pull_output(Level::Low).degrade(),
        ],
        [
            p0.p0_28.into_push_pull_output(Level::High).degrade(),
            p0.p0_11.into_push_pull_output(Level::High).degrade(),
            p0.p0_31.into_push_pull_output(Level::High).degrade(),
            p1.p1_05.into_push_pull_output(Level::High).degrade(),
            p0.p0_30.into_push_pull_output(Level::High).degrade(),
        ],
    )
}
```

Run the code normally and with release build:

```
cargo embed
cargo embed --release
```

## Interrupts

Add interrupt logic instead of busy waiting:

```rs
#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::PinState;
use hal::pac::interrupt;
use nrf52833_hal::rtc::RtcCompareReg;
use nrf52833_hal::rtc::RtcInterrupt;
use nrf52833_hal::Rtc;
use nrf52833_hal::{self as hal, gpio::Level};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const DISPLAY_CONTENT: [[bool; 5]; 5] = [
    [true, true, false, true, true],
    [true, true, false, true, true],
    [false, false, false, false, false],
    [true, false, false, false, true],
    [false, true, true, true, false],
];

// We need to share the RTC between the main execution thread and an interrupt, hence the mutex.
// They'll never be any contention though as interrupts cannot fire while there's a critical
// section. Also note that the Mutex here is from cortex_m and is designed to work
// only with single core processors.
static RTC: Mutex<RefCell<Option<Rtc<hal::pac::RTC0>>>> = Mutex::new(RefCell::new(None));

// Keep a flag to indicate that our timer has expired.
static TIMER_EXPIRED: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    // Here we use RTT. In practice, consider dfmt.
    // (see https://defmt.ferrous-systems.com/introduction)
    rtt_init_print!();

    rprintln!("Configuring display pins as outputs");
    let mut cp = hal::pac::CorePeripherals::take().unwrap();
    let p = hal::pac::Peripherals::take().unwrap();

    // Enable the low-power/low-frequency clock which is required by the RTC.
    let clocks = hal::clocks::Clocks::new(p.CLOCK);
    clocks.start_lfclk();

    let mut rtc = Rtc::new(p.RTC0, 0).unwrap();
    rtc.set_compare(RtcCompareReg::Compare0, 32768 / 300 - 1)
        .unwrap();
    rtc.enable_event(RtcInterrupt::Compare0);
    rtc.enable_interrupt(RtcInterrupt::Compare0, Some(&mut cp.NVIC));

    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    let (mut row_pins, mut col_pins) = as_outputs(port0, port1);

    rprintln!("Starting RTC");
    rtc.enable_counter();

    // Permit the interrupt to gain access to the RTC for the purpsoes of resetting etc.
    // We use the inner mutability of the RefCell to allow the interrupt to modify the
    // RTC state.
    cortex_m::interrupt::free(|cs| {
        RTC.borrow(cs).replace(Some(rtc));
    });

    rprintln!("Displaying content");
    let mut current_row = 0usize;
    loop {
        // Set current row to HIGH
        row_pins[current_row].set_high().unwrap();

        // Set columns to the appropriate values
        (0..DISPLAY_CONTENT[current_row].len()).for_each(|ix| {
            col_pins[ix]
                .set_state(if DISPLAY_CONTENT[current_row][ix] {
                    PinState::Low
                } else {
                    PinState::High
                })
                .unwrap();
        });

        while TIMER_EXPIRED.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
            != Ok(true)
        {
            // Go to sleep until we get an event (typically our RTC interrupt)
            cortex_m::asm::wfe();
        }

        // Reset columns to HIGH and current row to LOW before moving to next row
        (0..DISPLAY_CONTENT[current_row].len()).for_each(|ix| {
            col_pins[ix].set_high().unwrap();
        });
        row_pins[current_row].set_low().unwrap();

        rprintln!("Next row");
        current_row = (current_row + 1) % DISPLAY_CONTENT.len();
    }
}

fn as_outputs(
    p0: hal::gpio::p0::Parts,
    p1: hal::gpio::p1::Parts,
) -> ([impl OutputPin; 5], [impl OutputPin; 5]) {
    (
        // Find the pin connections for the Microbit in the schematics
        // (https://tech.microbit.org/hardware/schematic/#schematics--bom).
        [
            p0.p0_21.into_push_pull_output(Level::Low).degrade(),
            p0.p0_22.into_push_pull_output(Level::Low).degrade(),
            p0.p0_15.into_push_pull_output(Level::Low).degrade(),
            p0.p0_24.into_push_pull_output(Level::Low).degrade(),
            p0.p0_19.into_push_pull_output(Level::Low).degrade(),
        ],
        [
            p0.p0_28.into_push_pull_output(Level::High).degrade(),
            p0.p0_11.into_push_pull_output(Level::High).degrade(),
            p0.p0_31.into_push_pull_output(Level::High).degrade(),
            p1.p1_05.into_push_pull_output(Level::High).degrade(),
            p0.p0_30.into_push_pull_output(Level::High).degrade(),
        ],
    )
}

#[interrupt]
fn RTC0() {
    cortex_m::interrupt::free(|cs| {
        let rtc = RTC.borrow(cs).borrow();
        if let Some(rtc) = rtc.as_ref() {
            rtc.reset_event(RtcInterrupt::Compare0);
            rtc.clear_counter();
        }
    });

    TIMER_EXPIRED.store(true, Ordering::Relaxed);
}
```

Run the code.

## What next?

* Get good foundational knowledge of Rust in general.
* Create a minimal starter project for your specific microcontroller.
* Learn about important crates for embedded development (besides the ones mentioned above). Examples:
  * [_heapless_](https://crates.io/crates/heapless)
  * [_futures_](https://crates.io/crates/futures)
  * [_Embassy_](https://embassy.dev/)
  * [_RTIC_](https://rtic.rs/2/book/en/)
  * More inspiration at [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)
