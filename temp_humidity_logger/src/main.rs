extern crate rppal;
extern crate rppal_sys;

use rppal::gpio::{Gpio, InputPin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let pin = gpio.get(4).expect("Failed to get GPIO pin 4").into_input();
    
    loop {
        match read_dht11(&pin) {
            Ok((humidity, temperature)) => {
                println!("Humidity: {}%, Temperature: {}°C", humidity, temperature);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }

        // Delay for a while before reading again
        sleep(Duration::from_secs(2));
    }
}

fn read_dht11(pin: &InputPin) -> Result<(f32, f32), &'static str> {
    // Send start signal
    pin.set_mode(rppal::gpio::Mode::Output);
    pin.write(rppal::gpio::Level::Low);
    sleep(Duration::from_millis(18));
    pin.set_mode(rppal::gpio::Mode::Input);

    // Read response
    let (mut data, mut i) = ([0u8; 5], 0);
    let mut last_state = rppal::gpio::Level::High;

    for _ in 0..80 {
        let mut count = 0;
        while pin.read() == last_state {
            count += 1;
            if count >= 255 {
                return Err("Sensor response timeout");
            }
            sleep(Duration::from_micros(1));
        }

        last_state = pin.read();

        if (i % 8) != 0 {
            data[i / 8] <<= 1;
        }

        if count > 16 {
            data[i / 8] |= 1;
        }

        i += 1;
    }

    // Verify checksum
    if (data[0] as u16 + data[1] as u16 + data[2] as u16 + data[3] as u16) & 0xFF != data[4] as u16 {
        return Err("Checksum verification failed");
    }

    let humidity = data[0] as f32 + (data[1] as f32 * 0.1);
    let temperature = (data[2] & 0x7F) as f32 + ((data[3] as f32 & 0x7F) * 0.1);

    Ok((humidity, temperature))
}
