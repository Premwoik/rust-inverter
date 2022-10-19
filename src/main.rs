use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Trigger;

const GPIO_COUNTER_OUTPUT: u8 = 6;
const GPIO_COUNTER_INPUT: u8 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    
    let mut input = gpio.get(GPIO_COUNTER_INPUT)?.into_input_pullup();
    input.set_async_interrupt(Trigger::RisingEdge, |_| {println!("input");})?;

    let mut output = gpio.get(GPIO_COUNTER_OUTPUT)?.into_input_pullup();
    output.set_async_interrupt(Trigger::RisingEdge, |_| println!("output"))?;

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("loop");
    }
}
