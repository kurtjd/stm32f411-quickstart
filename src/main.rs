#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use stm32f4xx_hal::{pac::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let mut delay = cp.SYST.delay(&clocks);

    println!("BLINKY");
    loop {
        led.set_high();
        delay.delay_ms(500);
        led.set_low();
        delay.delay_ms(500);

        println!("BLINK");
    }
}
