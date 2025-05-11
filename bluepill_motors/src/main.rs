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

    let mut motor_in1 = Output::new(p.PC13, Level::High, Speed::Low);
    let mut motor_in2 = Output::new(p.PC14, Level::High, Speed::Low);

    loop {
        info!("fwd");
        motor_in1.set_level(Level::Low);
        motor_in2.set_high();
        Timer::after_millis(1000).await;
        return;

        //info!("low");
        //motor_in1.set_low();
        //Timer::after_millis(300).await;
    }
}