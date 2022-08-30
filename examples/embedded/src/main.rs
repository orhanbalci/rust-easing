#![no_main]
#![no_std]

use easer::functions::*;
use hal::{prelude::*, pwm, pwm::Pwm, Rtc};
use nrf52840_hal as hal;
use nrf52840_hal::gpio::Level;
use num_traits::Float;
use panic_reset as _;

// RTC will count at ~100 Hz
const RTC_PRESCALER: u32 = 327;
// Tick is ~10 ms
const TICK: f32 = (RTC_PRESCALER as f32 + 1.0) / 32768.0;
// Run animation for 5 seconds
const DURATION: f32 = 5.0;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    let led = port1.p1_13.into_push_pull_output(Level::Low);

    // Enable the low-power/low-frequency clock which is required by the RTC.
    let clocks = hal::clocks::Clocks::new(p.CLOCK);
    let _clocks = clocks.start_lfclk();

    let rtc = Rtc::new(p.RTC0, RTC_PRESCALER).unwrap();
    rtc.enable_counter();

    let pwm = Pwm::new(p.PWM0);
    pwm.set_period(1000u32.hz());
    pwm.set_max_duty(32767);
    pwm.set_output_pin(pwm::Channel::C0, led.degrade());
    pwm.enable();

    loop {
        let time = rtc.get_counter();
        let time_s = (time as f32) * TICK;

        let brightness = Bounce::ease_in_out(time_s % DURATION, 0.0, 1.0, DURATION);

        // Apply a gamma of 3.0 so that the brightness change is perceptable.
        pwm.set_duty_on_common((brightness.powi(3) * (pwm.get_max_duty() as f32)) as u16);
    }
}
