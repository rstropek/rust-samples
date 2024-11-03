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
