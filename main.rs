use async_std::task;
use sysfs_gpio::{Direction, Pin};
use std::time::Duration;

fn main() {
    task::block_on(async {
        let pin_num = 17; // GPIO pin 17

        let mut pin = Pin::new(pin_num);
        pin.export().expect("Failed to export GPIO pin");
        pin.set_direction(Direction::Out)
            .expect("Failed to set GPIO direction");

        println!("Running on: Raspberry Pi (or similar)");

        loop {
            // Send start signal to the DHT11 sensor
            pin.set_value(0).expect("Failed to set GPIO value");
            task::sleep(Duration::from_millis(18)).await;
            pin.set_value(1).expect("Failed to set GPIO value");
            task::sleep(Duration::from_micros(40)).await;

            // Switch to input mode for reading
            pin.set_direction(Direction::In)
                .expect("Failed to set GPIO direction");

            // Wait for the DHT11 sensor to respond
            while pin.get_value().expect("Failed to read GPIO value") == 1 {}
            while pin.get_value().expect("Failed to read GPIO value") == 0 {}

            // Read data from the DHT11 sensor
            let mut data = [0u8; 5];
            for i in 0..5 {
                data[i] = read_byte(&pin).await.expect("Error reading byte");
            }

            // Interpret the data
            let humidity = data[0] as f32;
            let temperature = data[2] as f32;

            println!("Temperature: {:.1}°C, Humidity: {:.1}%", temperature, humidity);

            task::sleep(Duration::from_secs(2)).await; // Delay between readings
        }
    });
}

async fn read_byte(pin: &Pin) -> Result<u8, Box<dyn std::error::Error>> {
    let mut byte = 0;
    for _ in 0..8 {
        while pin.get_value()? == 0 {
            task::sleep(Duration::from_micros(50)).await;
        }
        task::sleep(Duration::from_micros(50)).await;

        if pin.get_value()? == 1 {
            byte |= 1;
        }
        byte <<= 1;

        while pin.get_value()? == 1 {
            task::sleep(Duration::from_micros(50)).await;
        }
    }
    Ok(byte)
}
