use core::marker::PhantomData;
use crate::qei::SlaveMode::QuadratureEncoderModeX1CC1P;
use crate::spi::Pins;
use crate::timer::Timer;
use crate::stm32::TIM2;


/// SMS (Slave Mode Selection) register
#[derive(Copy, Clone, Debug)]
pub enum SlaveMode {
    /// Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
    EncoderMode1 = 0b0001,
    /// Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
    EncoderMode2 = 0b0010,
    /// Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the
    /// level of the other input.
    EncoderMode3 = 0b0011,
    /// Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and
    /// generates an update of the registers.
    ResetMode = 0b0100,
    /// Gated Mode - The counter clock is enabled when the trigger input (tim_trgi) is high.
    /// The counter stops (but is not reset) as soon as the trigger becomes low. Both start and
    /// stop of the counter are controlled.
    GatedMode = 0b0101,
    /// Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not
    /// reset). Only the start of the counter is controlled.
    TriggerMode = 0b0110,
    /// External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    ExternalClockMode1 = 0b0111,
    ///Combined reset + trigger mode - Rising edge of the selected trigger input (tim_trgi)
    /// reinitializes the counter, generates an update of the registers and starts the counter.
    CombinedReset = 0b1000,
    ///Combined gated + reset mode - The counter clock is enabled when the trigger input
    /// (tim_trgi) is high. The counter stops and is reset) as soon as the trigger becomes low.
    /// Both start and stop of the counter are controlled.
    CombinedGated = 0b1001,
    /// Encoder mode: Clock plus direction, x2 mode.
    EncoderModeClockPlusDirectionX2 = 0b1010,
    /// Encoder mode: Clock plus direction, x1 mode, tim_ti2fp2 edge sensitivity is set by
    EncoderModeClockPlusDirectionX1 = 0b1011,
    ///  Encoder mode: Directional Clock, x2 mode
    EncoderModeDirectionalClockX2 = 0b1100,
    ///  Encoder mode: Directional Clock, x1 mode, tim_ti1fp1 and tim_ti2fp2 edge sensitivity
    /// is set by CC1P and CC2P.
    EncoderModeDirectionalClockX1 = 0b1101,
    /// Quadrature encoder mode: x1 mode, counting on tim_ti1fp1 edges only, edge
    /// sensitivity is set by CC1P.
    QuadratureEncoderModeX1CC1P = 0b1110,
    /// Quadrature encoder mode: x1 mode, counting on tim_ti2fp2 edges only, edge
    /// sensitivity is set by CC2P
    QuadratureEncoderModeX1CC2P = 0b1111,
}

/// Quadrature Encoder Interface (QEI) options
///
/// The `Default` implementation provides a configuration for a 4-count pulse which counts from
/// 0-65535. The counter wraps back to 0 on overflow.
#[derive(Copy, Clone, Debug)]
pub struct QeiOptions {
    /// Encoder slave mode
    pub slave_mode: SlaveMode,

    /// Autoreload value
    /// This value allows the maximum count to be configured, up to 65535. Setting a lower value
    /// will overflow the counter to 0 sooner.
    pub auto_reload_value: u16,
}

impl Default for QeiOptions {
    fn default() -> Self {
        Self {
            slave_mode: SlaveMode::EncoderMode3,
            auto_reload_value: u16::MAX,
        }
    }
}

pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

impl Timer<TIM2> {
    pub fn qei<PINS>(self, pins: PINS, options: QeiOptions) -> Qei<TIM2, PINS> {
        // TODO: input channel timer options
        // TODO: enum for every register status (cc1s, cc2s ...)
        // TIMx_CCMR1 -> CC1S to map tim_ti1fp1 to tim_ti1 depending on options
        self.tim.ccmr1_input().write(|w| unsafe {
            w.cc1s().bits(0b01)
        });

        // TIMx_CCMR2 -> CC2S to map tim_ti2fp2 to tim_ti2 depending on options
        self.tim.ccmr1_input().write(|w| unsafe {
            w.cc2s().bits(0b01)
        });
        // TIMx_CCER -> CC1P and CC1NP to invert ( or not ) tim_ti1fp1 tim_ti1fp1 tim_ti1
        self.tim.ccer.write(|w| unsafe {
            w.cc1p().bit(false)
        });
        // TIMx_CCER -> CC2P and CC2NP to invert ( or not ) tim_ti2fp2 tim_ti2fp2 tim_ti2
        self.tim.ccer.write(|w| unsafe {
            w
                .cc2p().bit(false)
                .cc2np().bit(false)
        });
        // TIMx_SMCR -> SMS to configure slave mode
        self.tim.smcr.write(|w| unsafe {
            w
                .sms().bits(options.slave_mode as u8 & 0x07)
                .sms_3().bit((options.slave_mode as u8 & 0x08) != 0)
        });
        // TIMx_CR1 -> CEN activate counter
        self.tim.cr1.write(|w| unsafe {
            w.cen().bit(true)
        });
        Qei { tim: self.tim, pins }
    }
}