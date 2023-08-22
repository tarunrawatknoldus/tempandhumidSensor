extern crate rppal;

use std::thread::sleep;
use std::time::Duration;
use rppal::gpio::{Gpio, Level};
use rppal::system::DeviceInfo;

const DHT_PIN: u8 = 4;

fn main() {
    println!("Raspberry Pi Temperature and Humidity Monitoring");

    let gpio = Gpio::new().expect("Failed to initialize GPIO.");
    let pin = gpio.get(DHT_PIN).expect("Failed to get GPIO pin.").into_input_pullup();

    loop {
        if let Ok(data) = read_dht_data(&pin) {
            let (temperature, humidity) = parse_dht_data(data);
            println!("Temperature: {:.1}°C, Humidity: {:.1}%", temperature, humidity);
        } else {
            println!("Failed to read data from DHT11.");
        }

        sleep(Duration::from_secs(2));
    }
}

fn read_dht_data(pin: &rppal::gpio::InputPin) -> Result<[u8; 5], rppal::gpio::Error> {
    let mut data = [0u8; 5];

    // Send start signal
    pin.set_mode(rppal::gpio::Mode::Output);
    pin.write(Level::Low);
    std::thread::sleep(Duration::from_millis(18));
    pin.write(Level::High);
    std::thread::sleep(Duration::from_micros(40));
    pin.set_mode(rppal::gpio::Mode::Input);

    // Read data bits
    let mut prev_level = Level::High;
    let mut bit_idx = 0;
    for _ in 0..40 {
        let level = pin.read();
        if prev_level == Level::High && level == Level::Low {
            data[bit_idx / 8] <<= 1;
            prev_level = level;
        } else if prev_level == Level::Low && level == Level::High {
            data[bit_idx / 8] |= 1;
            prev_level = level;
            bit_idx += 1;
        }
        std::thread::sleep(Duration::from_micros(50));
    }

    Ok(data)
}

fn parse_dht_data(data: [u8; 5]) -> (f32, f32) {
    let humidity = data[0] as f32;
    let temperature = data[2] as f32 + (data[3] as f32 / 10.0);
    (temperature, humidity)
}
