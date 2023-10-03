use simple_dht11::dht11::Dht11;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() {
    // Create a new Dht11 instance, specifying the GPIO pin where the sensor is connected (BCM numbering).
    let mut dht11 = Dht11::new(27); // Note this is BCM

    // Use the Dht11 instance to get sensor reading data.
    let response = dht11.get_reading();

    // Print the temperature and humidity readings to the console.
    println!("Temperature: {}°C", response.temperature);
    println!("Humidity: {}%", response.humidity);

    // Store the data in a local file.
    if let Err(err) = save_to_file(response) {
        eprintln!("Error: {}", err);
    }
}

fn save_to_file(response: simple_dht11::dht11::Reading) -> io::Result<()> {
    // Open or create a file named "sensor_data.txt" for appending.
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("sensor_data.txt")?;

    // Format the data as a string and write it to the file.
    let data = format!("Temperature: {}°C, Humidity: {}%\n", response.temperature, response.humidity);
    file.write_all(data.as_bytes())?;

    Ok(())
}
