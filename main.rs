extern crate rppal;

use rppal::gpio::{Gpio, Level};
use rppal::system::DeviceInfo;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Running on: {:?}", DeviceInfo::new().unwrap());

    let gpio = Gpio::new().expect("Failed to initialize GPIO");

    let mut pin = gpio
        .get(17)
        .expect("Failed to get GPIO pin")
        .into_output(); // Initialize as OutputPin

    loop {
        send_start_signal(&mut pin);
        let data = read_data(&gpio, 17);

        let humidity = data[0] as f32;
        let temperature = data[2] as f32;

        println!("Temperature: {:.1}°C, Humidity: {:.1}%", temperature, humidity);

        sleep(Duration::from_secs(2)); // Delay between readings
    }
}

fn send_start_signal(pin: &mut rppal::gpio::OutputPin) {
    pin.set_low();
    sleep(Duration::from_millis(18));
    pin.set_high();
    sleep(Duration::from_micros(40));
}

fn read_data(gpio: &Gpio, pin_num: u8) -> [u8; 5] {
    let mut data = [0u8; 5];
    let mut pin = gpio.get(pin_num).unwrap().into_input();

    for i in 0..5 {
        data[i] = read_byte(&pin);
    }

    data
}

fn read_byte(pin: &rppal::gpio::InputPin) -> u8 {
    let mut byte = 0;

    for _ in 0..8 {
        while pin.read() == Level::Low {}
        sleep(Duration::from_micros(50));

        if pin.read() == Level::High {
            byte |= 1;
        }
        byte <<= 1;

        while pin.read() == Level::High {}
    }

    byte
}
