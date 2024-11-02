#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello world!");

    const ROW_PINS: [(u8, u32); 5] = [(0, 21), (0, 22), (0, 15), (0, 24), (0, 19)];
    const COL_PINS: [(u8, u32); 5] = [(0, 28), (0, 11), (0, 31), (1, 5), (0, 30)];

    const GPIO0_BASE_ADDR: u32 = 0x5000_0000;
    const GPIO1_BASE_ADDR: u32 = 0x5000_0300;

    const CONFIG_REG_OFFSET: u32 = 0x700;
    const REGISTER_LENGTH: u32 = 0x4;
    const OUT_SET_REG_OFFSET: u32 = 0x508;
    const OUT_CLR_REG_OFFSET: u32 = 0x50C;

    const fn pin_config_address(port: u8, pin: u32) -> *mut u32 {
        return (if port == 0 { GPIO0_BASE_ADDR } else { GPIO1_BASE_ADDR } + CONFIG_REG_OFFSET + (pin * REGISTER_LENGTH)) as *mut u32;
    }

    const fn out_set_address(port: u8) -> *mut u32 {
        return (if port == 0 { GPIO0_BASE_ADDR } else { GPIO1_BASE_ADDR } + OUT_SET_REG_OFFSET) as *mut u32;
    }

    const fn out_clr_address(port: u8) -> *mut u32 {
        return (if port == 0 { GPIO0_BASE_ADDR } else { GPIO1_BASE_ADDR } + OUT_CLR_REG_OFFSET) as *mut u32;
    }

    const DIR_OUTPUT_POS: u32 = 0; // Bit in configuration register to choose between input and output (page 269)
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS; // 1 means output, 0 means input (page 269) -> we need 1

    // Configure the direction of the row and col pins to output
    unsafe {
        for (port, pin) in ROW_PINS {
            let addr = pin_config_address(port, pin);
            write_volatile(addr, PINCNF_DRIVE_LED);
        }

        for (port, pin) in COL_PINS {
            let addr = pin_config_address(port, pin);
            write_volatile(addr, PINCNF_DRIVE_LED);
        }
    }

    unsafe {
        // Initialize: rows LOW (off) and columns HIGH (off)
        for (port, pin) in ROW_PINS {
            write_volatile(out_clr_address(port), 1 << pin);  // Keep rows LOW initially
        }

        for (port, pin) in COL_PINS {
            write_volatile(out_set_address(port), 1 << pin);  // Keep columns HIGH initially
        }
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
                        write_volatile(out_set_address(ROW_PINS[row_idx].0), 1 << ROW_PINS[row_idx].1);
                        
                        for (col_idx, visible) in row.into_iter().enumerate() {
                            if visible {
                                // For visible pixels, set column to LOW
                                write_volatile(out_clr_address(COL_PINS[col_idx].0), 1 << COL_PINS[col_idx].1);
                            } else {
                                // For invisible pixels, keep column HIGH
                                write_volatile(out_set_address(COL_PINS[col_idx].0), 1 << COL_PINS[col_idx].1);
                            }
                        }
                        
                        for _ in 0..100 {
                            nop();
                        }
                        
                        // Reset columns to HIGH before moving to next row
                        for (port, pin) in COL_PINS {
                            write_volatile(out_set_address(port), 1 << pin);
                        }
                        
                        // Set current row back to LOW
                        write_volatile(out_clr_address(ROW_PINS[row_idx].0), 1 << ROW_PINS[row_idx].1);
                    }
                }

            } else {
                // Turn off all LEDs by setting all rows LOW and all columns HIGH
                for (port, pin) in ROW_PINS {
                    write_volatile(out_clr_address(port), 1 << pin);
                }
                for (port, pin) in COL_PINS {
                    write_volatile(out_set_address(port), 1 << pin);
                }
                
                // Add delay for the off state
                for _ in 0..300000 {
                    nop();
                }
            }
        }

        is_on = !is_on;
    }
}
