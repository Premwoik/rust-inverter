extern crate reqwest;

use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::CONTENT_TYPE;
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use rppal::gpio::Gpio;
use rppal::gpio::Trigger;

const GPIO_COUNTER_OUTPUT: u8 = 6;
const GPIO_COUNTER_INPUT: u8 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;

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
            write_mes = Instant::now();
            let ic = input_counter.clone();
            let oc = output_counter.clone();
            thread::spawn(move || write_influxdb(ic, oc));
        }
    }
}

fn write_influxdb(input_counter: u32, output_counter: u32) -> () {
    let data = format!("energy_meter,meter_id=331 usage={input_counter}\nenergy_meter,meter_id=332 usage={output_counter}");
    let client = Client::new();
    let url = "http://192.168.2.100:8086/api/v2/write?bucket=default&org=SmartHome";
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer fUqLk2fqqb62jyE2PYNpx1mNbu38s75SKN7thO1nKpNqf2vRzb24QWopAlUjh-WM54xJ2KJA2_jXDYzGSlPKDQ=="));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    match client.post(url).headers(headers).body(data).send() {
        Ok(response) => assert!(
            response.status().is_success(),
            "Something went wrong when writing to InfluxDB"
        ),
        _ => println!("Something went wrong when trying write to InfluxDB"),
    };
}
