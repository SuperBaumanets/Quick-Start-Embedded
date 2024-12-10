#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{
    pac,
    prelude::*,
    delay::Delay
};


#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let _ = device.SYSCFG.constrain();

    let gpiod = device.GPIOD.split();

    let mut led = gpiod.pd13.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &clocks);

    loop {
        led.toggle();
        delay.delay_ms(200u16);
    }
}