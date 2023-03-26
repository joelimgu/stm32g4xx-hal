// #![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::peripheral::NVIC;
use hal::delay::DelayFromCountDownTimer;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use hal::timer::Timer;
use stm32g4xx_hal as hal;
use hal::gpio::AF2;
use heapless::Vec;


use cortex_m_rt::entry;
use stm32g4xx_hal::stm32::{Interrupt, interrupt};
use cortex_m_semihosting::{hprint, hprintln};
use stm32g4::stm32g431::tim1::AF2;
use stm32g4::stm32g431::TIM4;
use hal::gpio::Alternate;
use stm32g4xx_hal::gpio::gpiob::{PB6, PB7};
use stm32g4xx_hal::qei::{Qei, QeiOptions};
use stm32g4xx_hal::timer::Event;

#[macro_use]
mod utils;

static mut STIM3: Option<Qei<TIM4, (PB6<Alternate<2>>, PB7<Alternate<2>>)>> = None;

unsafe fn clear_tim2interrupt_bit() {
    (*stm32g4::stm32g431::TIM2::ptr())
        .sr
        .write(|w| w.uif().clear_bit());
}
#[interrupt]
fn TIM2() {
    static mut v: Vec<u16, 15000> = Vec::new();
    cortex_m::interrupt::free(|cs| unsafe {
        match STIM3.as_mut() {
            Some(stim) => {
                if v.is_full() {
                    hprintln!("FIN:{:?}", v);
                } else {
                    // hprintln!("pushed:{:?}", v);
                    v.push(stim.read() as u16).expect("Cant push into vec");
                }
            }
            _ => {
                hprintln!("e not present");
            }
        }
    });
    unsafe { clear_tim2interrupt_bit() }
}


#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let mut rcc = dp.RCC.freeze(Config::hsi());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpioa.pa4.into_push_pull_output();
    let ch1: PB6<Alternate<{ AF2 }>> = gpiob.pb6.into_alternate();
    let ch2: PB7<Alternate<{ AF2 }>> = gpiob.pb7.into_alternate();


    let tim = Timer::new(dp.TIM2, &rcc.clocks);
    let mut tim2 = tim.
        start_count_down(5.khz());
    tim2.listen(Event::TimeOut);

    let qei: Qei<TIM4, (PB6<Alternate<2>>, PB7<Alternate<2>>)> = Timer::new(dp.TIM4, &rcc.clocks)
        .qei((ch1,ch2),QeiOptions::default());
    unsafe {
        STIM3.replace(qei);
    }

    // let mut tim2 = Timer::new(dp.TIM3, &rcc.clocks)
    //     .start_count_down(2.khz());
    // tim2.listen(Event::TimeOut);
    // tim2.release();
    unsafe{NVIC::unmask(Interrupt::TIM2)}
    let mut delay_syst = cp.SYST.delay(&rcc.clocks);

    // let mut delay_tim2 = DelayFromCountDownTimer::new(timer2.start_count_down(100.ms()));

    led.set_high().unwrap();

    loop {
        // delay_syst.delay_ms(1000_u32);
        // unsafe {
        //     if let Some(stim) = STIM3.as_ref() {
        //         hprintln!("{:?}", stim.read());
        //     }
        // }


        // hprintln!("Value: {}", qei.read());
    }
}
