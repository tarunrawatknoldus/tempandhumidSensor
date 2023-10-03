use rppal::gpio::{Gpio, InputPin, Level};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Get the BCM pin number for the GPIO pin connected to the DHT11 sensor.
    let pin_num = 27;

    // Create a GPIO instance.
    let gpio = Gpio::new().expect("Failed to create GPIO instance");

    // Get the pin associated with the BCM pin number.
    let pin = gpio
        .get(pin_num)
        .expect("Failed to get pin")
        .into_input_pullup();

    // Read sensor data.
    let (temperature, humidity) = read_dht11(&pin).expect("Failed to read DHT11 sensor");

    // Print the temperature and humidity readings to the console.
    println!("Temperature: {}°C", temperature);
    println!("Humidity: {}%", humidity);

    // Store the data in a local file.
    if let Err(err) = save_to_file(temperature, humidity) {
        eprintln!("Error: {}", err);
    }
}

fn read_dht11(pin: &InputPin) -> Result<(f32, f32), rppal::gpio::Error> {
    // Send start signal by pulling the pin low for at least 18ms.
    pin.set_mode(rppal::gpio::Mode::Output)?;
    pin.set_value(Level::Low);
    sleep(Duration::from_millis(18));

    // Switch the pin to input mode and wait for the sensor response.
    pin.set_mode(rppal::gpio::Mode::Input)?;
    sleep(Duration::from_micros(40));

    // Read the response from the sensor.
    let mut data = [0u8; 5];
    for i in 0..40 {
        sleep(Duration::from_micros(1));
        if pin.is_set_high()? {
            let index = i / 8;
            data[index] <<= 1;
            data[index] |= 1;
        } else {
            let index = i / 8;
            data[index] <<= 1;
        }
    }

    // Verify the checksum of the received data.
    let checksum = (data[0] as u16)
        .wrapping_add(data[1] as u16)
        .wrapping_add(data[2] as u16)
        .wrapping_add(data[3] as u16);
    if checksum != (data[4] as u16) {
        return Err(rppal::gpio::Error::Unknown);
    }

    // Parse the temperature and humidity values from the received data.
    let temperature = (data[2] as f32) + (data[3] as f32) / 10.0;
    let humidity = (data[0] as f32) + (data[1] as f32) / 10.0;

    Ok((temperature, humidity))
}

fn save_to_file(temperature: f32, humidity: f32) -> io::Result<()> {
    // Open or create a file named "sensor_data.txt" for appending.
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("sensor_data.txt")?;

    // Format the data as a string and write it to the file.
    let data = format!("Temperature: {}°C, Humidity: {}%\n", temperature, humidity);
    file.write_all(data.as_bytes())?;

    Ok(())
}
