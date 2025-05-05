#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, OutputType, Speed };
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("Hello World!");
    
    let ch1_pin = PwmPin::new_ch1(p.PB6, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM4, Some(ch1_pin), None, None, None, khz(20), Default::default());

    let mut dir = Output::new(p.PB7, Level::Low, Speed::Low);

    let mut ch1 = pwm.ch1();
    ch1.enable();

    loop {
        ch1.set_duty_cycle_fully_off();
        Timer::after_millis(300).await;
        ch1.set_duty_cycle_fraction(1, 4);
        Timer::after_millis(300).await;
        ch1.set_duty_cycle_fraction(1, 2);
        Timer::after_millis(300).await;
        ch1.set_duty_cycle(ch1.max_duty_cycle() - 1);
        Timer::after_millis(300).await;


    }
}