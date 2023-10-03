use simple_dht11::dht11::Dht11;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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

        // Load existing JSON data from the file
        let mut existing_data = load_from_json_file()?;

        // Create a SensorData struct with the new reading
        let new_data = SensorData {
            temperature: response.temperature,
            humidity: response.humidity,
        };

        // Append the new reading to the existing data
        existing_data.push(new_data);

        // Serialize the updated data to JSON
        let json_data = serde_json::to_string(&existing_data)?;

        // Write the updated JSON data back to the file
        save_to_json_file(&json_data)?;

        // Sleep for 1 second before the next reading
        sleep(Duration::from_secs(1));
    }
}

fn load_from_json_file() -> Result<Vec<SensorData>, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("sensor_data.json")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let data: Vec<SensorData> = serde_json::from_str(&contents)?;

    Ok(data)
}

fn save_to_json_file(data: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true) // Append mode
        .open("sensor_data.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
