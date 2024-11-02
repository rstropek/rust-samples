#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use nrf52833_pac::interrupt;

const RTC_PERIPHERAL_ID: u32 = 0x11;
const RTC1_BASE: u32 = 0x40000000 + RTC_PERIPHERAL_ID * 0x1000;
const LFCLK_START: u32 = 0x40000008;

// See also https://developer.arm.com/documentation/dui0497/a/cortex-m0-peripherals/nested-vectored-interrupt-controller/interrupt-set-enable-register?lang=en
const NVIC_ISER: u32 = 0xE000E100;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Started");
    
    unsafe {
        // Enable LFCLK
        *(LFCLK_START as *mut u32) = 1;
        rprintln!("Pre-Event");
        while (*(0x40000104 as *const u32) & 1) == 0 {}
        rprintln!("Event");
    
        let rtc1 = RTC1_BASE as *mut u32;
        *rtc1.add(0x004 / 4) = 0;              // Stop RTC
        *rtc1.add(0x508 / 4) = 0xFFF;      // PRESCALER: This is correct (32768 - 1)
        *rtc1.add(0x540 / 4) = 8;          // COMPARE[0]: Should be 1, not 32768
        *rtc1.add(0x304 / 4) = 1 << 16;          // Enable COMPARE[0] interrupt
        
        *rtc1.add(0 / 4) = 1;              // Start RTC
        // cortex_m::asm::dmb();              // Add memory barrier
        *(NVIC_ISER as *mut u32) |= 1 << RTC_PERIPHERAL_ID;  // Enable RTC1 IRQ in NVIC
    }
    
    loop {
        nop();
    }
}

#[interrupt]
fn RTC1() {
    // Code to execute on interrupt
    rprintln!("RTC1");
    
    // Clear the event
    unsafe {
        let rtc1 = RTC1_BASE as *mut u32;
        *rtc1.add(0x140 / 4) = 0;  // Clear EVENTS_COMPARE[0]
        *rtc1.add(0x008 / 4) = 1;          // COMPARE[0]: Should be 1, not 32768
    }
}
