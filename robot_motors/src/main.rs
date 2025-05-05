#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led1 = Output::new(p.PD12, Level::High, Speed::Low);
    let mut led2 = Output::new(p.PD13, Level::High, Speed::Low);
    let mut led3 = Output::new(p.PD14, Level::High, Speed::Low);
    let mut led4 = Output::new(p.PD15, Level::High, Speed::Low);

    loop {
        info!("led1 high");
        led1.set_high();
        Timer::after_millis(300).await;

        info!("led1 low");
        led1.set_low();
        Timer::after_millis(300).await;

        info!("led2 high");
        led2.set_high();
        Timer::after_millis(300).await;

        info!("led2 low");
        led2.set_low();
        Timer::after_millis(300).await;

        info!("led3 high");
        led3.set_high();
        Timer::after_millis(300).await;

        info!("led3 low");
        led3.set_low();
        Timer::after_millis(300).await;

        info!("led4 high");
        led4.set_high();
        Timer::after_millis(300).await;
        info!("led4 low");
        led4.set_low();
        Timer::after_millis(300).await;


    }
}