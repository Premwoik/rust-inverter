mod inverter_api;

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;

const GPIO_COUNTER_OUTPUT: u8 = 6;
const GPIO_COUNTER_INPUT: u8 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;

    gpio.get(GPIO_COUNTER_OUTPUT)?
        .into_input_pullup()
        .set_async_interrupt(Trigger::RisingEdge, |_: Level| -> () {
            println!("Read output")
        })?;
    gpio.get(GPIO_COUNTER_INPUT)?
        .into_input_pullup()
        .set_async_interrupt(Trigger::RisingEdge, |_: Level| -> () {
            println!("Read input")
        })?;

    loop {
        thread::sleep(Duration::from_secs(5))
    }
}
