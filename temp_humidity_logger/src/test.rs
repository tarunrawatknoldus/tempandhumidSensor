use simple_dht11::dht11::Dht11;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct SensorData {
    temperature: f32,
    humidity: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut dht11 = Dht11::new(27); // Note this is BCM

    loop {
        let response = dht11.get_reading();

        println!("Temperature: {}°C", response.temperature);
        println!("Humidity: {}%", response.humidity);

        // Create a SensorData struct
        let sensor_data = SensorData {
            temperature: response.temperature,
            humidity: response.humidity,
        };

        // Serialize the data to JSON
        let json_data = serde_json::to_string(&sensor_data)?;

        // Write JSON data to a file
        if let Err(err) = save_to_json_file(&json_data) {
            eprintln!("Error writing data to JSON file: {}", err);
        }

        // Sleep for 1 second before the next reading
        sleep(Duration::from_secs(1));
    }
}

fn save_to_json_file(data: &str) -> Result<(), std::io::Error> {
    let mut file = File::create("sensor_data.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
