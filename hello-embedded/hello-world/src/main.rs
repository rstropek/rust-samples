#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

enum Port {
    P0 = 0,
    P1 = 1,
}

impl Port {
    const fn base_address(&self) -> u32 {
        const GPIO0_BASE_ADDR: u32 = 0x5000_0000;
        const GPIO1_BASE_ADDR: u32 = 0x5000_0300;

        match self {
            Port::P0 => GPIO0_BASE_ADDR,
            Port::P1 => GPIO1_BASE_ADDR,
        }
    }

    const fn config_address(&self, pin: u32) -> *mut u32 {
        const CONFIG_REG_OFFSET: u32 = 0x700;
        const REGISTER_LENGTH: u32 = 0x4;

        (self.base_address() + CONFIG_REG_OFFSET + (pin * REGISTER_LENGTH)) as *mut u32
    }

    const fn out_set_address(&self) -> *mut u32 {
        const OUT_SET_REG_OFFSET: u32 = 0x508;
        (self.base_address() + OUT_SET_REG_OFFSET) as *mut u32
    }

    const fn out_clr_address(&self) -> *mut u32 {
        const OUT_CLR_REG_OFFSET: u32 = 0x50C;
        (self.base_address() + OUT_CLR_REG_OFFSET) as *mut u32
    }
}

struct Pin {
    port: Port,
    pin: u32,
}

impl Pin {
    #[inline(always)]
    unsafe fn set_high(&self) {
        write_volatile(self.port.out_set_address(), 1 << self.pin);
    }

    #[inline(always)]
    unsafe fn set_low(&self) {
        write_volatile(self.port.out_clr_address(), 1 << self.pin);
    }

    #[inline(always)]
    unsafe fn configure_as_output(&self) {
        const DIR_OUTPUT_POS: u32 = 0; // Bit in configuration register to choose between input and output (page 269)
        const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS; // 1 means output, 0 means input (page 269) -> we need 1
        write_volatile(self.port.config_address(self.pin), PINCNF_DRIVE_LED);
    }
}

const ROW_PINS: [Pin; 5] = [
    Pin {
        port: Port::P0,
        pin: 21,
    },
    Pin {
        port: Port::P0,
        pin: 22,
    },
    Pin {
        port: Port::P0,
        pin: 15,
    },
    Pin {
        port: Port::P0,
        pin: 24,
    },
    Pin {
        port: Port::P0,
        pin: 19,
    },
];

const COL_PINS: [Pin; 5] = [
    Pin {
        port: Port::P0,
        pin: 28,
    },
    Pin {
        port: Port::P0,
        pin: 11,
    },
    Pin {
        port: Port::P0,
        pin: 31,
    },
    Pin {
        port: Port::P1,
        pin: 5,
    },
    Pin {
        port: Port::P0,
        pin: 30,
    },
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Started");

    blinking_cross();
}

fn blinking_cross() -> ! {
    // Configure the direction of the row and col pins to output
    unsafe {
        ROW_PINS.iter().for_each(|pin| pin.configure_as_output());
        COL_PINS.iter().for_each(|pin| pin.configure_as_output());
    }

    unsafe {
        // Initialize: rows LOW (off) and columns HIGH (off)
        ROW_PINS.iter().for_each(|pin| pin.set_low());
        COL_PINS.iter().for_each(|pin| pin.set_high());
    }

    const DISPLAY: [[bool; 5]; 5] = [
        [true, false, false, false, true],
        [false, true, false, true, false],
        [false, false, true, false, false],
        [false, true, false, true, false],
        [true, false, false, false, true],
    ];

    let mut is_on = false;
    loop {
        unsafe {
            if is_on {
                for _ in 0..500 {
                    for (row_idx, row) in DISPLAY.into_iter().enumerate() {
                        // Set current row to HIGH (active)
                        ROW_PINS[row_idx].set_high();

                        for (col_idx, visible) in row.into_iter().enumerate() {
                            if visible {
                                COL_PINS[col_idx].set_low();
                            } else {
                                COL_PINS[col_idx].set_high();
                            }
                        }

                        for _ in 0..100 {
                            nop();
                        }

                        // Reset columns to HIGH before moving to next row
                        COL_PINS.iter().for_each(|pin| pin.set_high());

                        // Set current row back to LOW
                        ROW_PINS[row_idx].set_low();
                    }
                }
            } else {
                // Turn off all LEDs by setting all rows LOW and all columns HIGH
                ROW_PINS.iter().for_each(|pin| pin.set_low());
                COL_PINS.iter().for_each(|pin| pin.set_high());

                // Add delay for the off state
                for _ in 0..300000 {
                    nop();
                }
            }
        }

        is_on = !is_on;
        rprintln!("is_on: {}", is_on);
    }
}