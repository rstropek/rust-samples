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

    // The LED matrix is organized into rows (ROW1 to ROW5) and columns (COL1 to COL5). Each row and column represents
    // connections to LEDs in a grid formation, where each intersection can turn on or off an LED by controlling the flow of current.
    // In a 5x5 matrix like this, there are 25 LEDs, arranged in rows and columns. Each LED is placed at an intersection of a row and a column.
    // By setting a row to a high (positive) voltage and a column to low (grounded), current can flow through the corresponding LED 
    // in that row and column, causing it to light up.
    // (see also Microbit Schematic at https://tech.microbit.org/hardware/schematic/, page 2

    // In our example, we will use ROW1 and COL1 to turn on an LED. From the schematics (page 3), we can see that 
    // ROW1 is connected to Port 0/Pin 21 and COL1 is connected to Port 0/Pin 28.

    // Configuration of GPIO happens in the PIN_CNF registers. The data sheet for nRF52833
    // (https://docs-be.nordicsemi.com/bundle/ps_nrf52833/attach/nRF52833_PS_v1.7.pdf?_LANG=enus) tells us that the
    // base address for PIN_CNF registers is 0x5000_0000 (page 231). The offset for GPIO21 is 0x754 and for GPIO28 it is 0x770
    // (page 232). We control the value of the pins using the OUT register. To set pin 21 high, we need to 
    // write 1 to bit 21 of the OUT register (page 232). The offset for the OUT register is 0x504 (page 231).

    const GPIO0_PINCNF21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    const GPIO0_PINCNF28_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0; // Bit in configuration register to choose between input and output (page 269)
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS; // 1 means output, 0 means input (page 269) -> we need 1

    // Configure the direction of the pins to output
    unsafe {
        // Set the direction of the pins 21 and 28 to output
        write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_DRIVE_LED);
        write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_DRIVE_LED);
    }

    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const GPIO0_OUT_ROW1_POS: u32 = 21; // Bit in the OUT register to control GPIO21
    let mut is_on = false;
    loop {
        rprintln!("Echo...");
        unsafe {
            // Set the value of the OUT register to turn on or off the LED
            // We do not set the value for GPIO28 because we want it to be off
            write_volatile(GPIO0_OUT_ADDR, (is_on as u32) << GPIO0_OUT_ROW1_POS);
        }

        for _ in 0..100_000 {
            nop();
        }

        is_on = !is_on;
    }
}
