// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use hal::delay::DelayFromCountDownTimer;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use hal::timer::Timer;
use stm32g4xx_hal as hal;
use hal::gpio::AF2;


use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use stm32g4::stm32g431::tim1::AF2;
use hal::gpio::Alternate;
use stm32g4xx_hal::gpio::gpiob::{PB6, PB7};
use stm32g4xx_hal::qei::QeiOptions;
use stm32g4xx_hal::timer::Event;

#[macro_use]
mod utils;

#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let mut rcc = dp.RCC.freeze(Config::hsi());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();
    let ch1: PB6<Alternate<{ AF2 }>> = gpiob.pb6.into_alternate();
    let ch2: PB7<Alternate<{ AF2 }>> = gpiob.pb7.into_alternate();


    let tim = Timer::new(dp.TIM2, &rcc.clocks);
    let mut tim2 = tim.start_count_down(100.hz());
    tim2.listen(Event::TimeOut);

    let qei = Timer::new(dp.TIM4, &rcc.clocks)
        .qei((ch1,ch2),QeiOptions::default());


    // let mut delay_syst = cp.SYST.delay(&rcc.clocks);

    // let mut delay_tim2 = DelayFromCountDownTimer::new(timer2.start_count_down(100.ms()));

    loop {
        // hprintln!("Value: {}", qei.read());
    }
}
