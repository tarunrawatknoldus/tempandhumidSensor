extern crate rppal;

use std::thread::sleep;
use std::time::Duration;
use rppal::gpio::{Gpio, InputPin};
use rppal::system::DeviceInfo;

const DHT_PIN: u8 = 4; // GPIO Pin number connected to DHT11 data pin

fn main() {
    println!("Raspberry Pi Temperature and Humidity Monitoring");

    let gpio = Gpio::new().expect("Failed to initialize GPIO.");
    let pin = gpio.get(DHT_PIN).expect("Failed to get GPIO pin.").into_input();

    loop {
        // Read data from DHT11
        if let Ok(data) = read_dht_data(&pin) {
            let (temperature, humidity) = parse_dht_data(data);
            println!("Temperature: {:.1}°C, Humidity: {:.1}%", temperature, humidity);
        } else {
            println!("Failed to read data from DHT11.");
        }

        sleep(Duration::from_secs(2));
    }
}

fn read_dht_data(pin: &InputPin) -> Result<[u8; 5], rppal::gpio::Error> {
    // Initialize variables to collect data bits
    let mut data = [0u8; 5];
    let mut bit_index = 0;
    let mut current_byte = 0u8;

    // Generate start signal
    let mut last_state = rppal::gpio::Level::High;
    for _ in 0..85 {
        let level = pin.read();
        if level == rppal::gpio::Level::Low && last_state == rppal::gpio::Level::High {
            // Detect falling edge as a start signal
            break;
        }
        last_state = level;
        std::thread::sleep(Duration::from_micros(2));
    }

    // Read data bits
    for _ in 0..40 {
        let level = pin.read();
        if level == rppal::gpio::Level::High {
            current_byte |= 1 << (7 - bit_index);
        }
        bit_index += 1;
        if bit_index == 8 {
            data[bit_index / 8 - 1] = current_byte;
            current_byte = 0;
            bit_index = 0;
        }
        std::thread::sleep(Duration::from_micros(1));
    }

    Ok(data)
}

fn parse_dht_data(data: [u8; 5]) -> (f32, f32) {
    let humidity = data[0] as f32;
    let temperature = data[2] as f32 + (data[3] as f32 / 10.0);
    (temperature, humidity)
}
