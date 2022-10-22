use std::error::Error;
use std::time::Duration;
use std::time::Instant;
use log::info;

use rppal::gpio::Gpio;
use rppal::gpio::Trigger;

const GPIO_COUNTER_OUTPUT: u8 = 6;
const GPIO_COUNTER_INPUT: u8 = 5;

mod influxdb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let client = influxdb::influx_new_client();

    let mut input = gpio.get(GPIO_COUNTER_INPUT)?.into_input_pullup();
    input.set_interrupt(Trigger::RisingEdge).ok();

    let mut output = gpio.get(GPIO_COUNTER_OUTPUT)?.into_input_pullup();
    output.set_interrupt(Trigger::RisingEdge).ok();

    let pins = [&input, &output];
    let mut output_counter = 0;
    let mut input_counter = 0;

    let mut input_mes = Instant::now();
    let mut output_mes = Instant::now();
    let mut write_mes = Instant::now();

    loop {
        match gpio.poll_interrupts(&pins, true, Some(Duration::from_secs(1))) {
            Ok(Some((pin, _))) if pin.pin() == GPIO_COUNTER_INPUT => {
                if input_mes.elapsed().as_millis() > 200 {
                    input_counter += 1;
                    input_mes = Instant::now();
                }
            }
            Ok(Some((pin, _))) if pin.pin() == GPIO_COUNTER_OUTPUT => {
                if output_mes.elapsed().as_millis() > 200 {
                    output_counter += 1;
                    output_mes = Instant::now();
                }
            }
            _ => (),
        }
        if write_mes.elapsed().as_secs() >= 300 {
            info!("Writing data from counters to InfluxDB...");

            influxdb::write_io_energy_counters(&client, input_counter, output_counter);
            write_mes = Instant::now();
            input_counter = 0;
            output_counter = 0;
        }
    }
}
