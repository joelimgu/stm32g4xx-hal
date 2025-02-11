//This example puts the timer in PWM mode using the specified pin with a frequency of 100Hz and a duty cycle of 50%.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::gpio::gpioa::PA8;
use hal::gpio::Alternate;
use hal::gpio::AF6;
use hal::prelude::*;
use hal::stm32;
use stm32g4xx_hal as hal;
mod utils;
extern crate cortex_m_rt as rt;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);
    let pin: PA8<Alternate<AF6>> = gpioa.pa8.into_alternate();

    let mut pwm = dp.TIM1.pwm(pin, 100.hz(), &mut rcc);

    pwm.set_duty(pwm.get_max_duty() / 2);
    pwm.enable();

    loop {
        cortex_m::asm::nop()
    }
}
