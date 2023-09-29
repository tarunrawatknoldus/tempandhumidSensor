use simple_dht11::dht11::Dht11;
use reqwest::Client;
use std::error::Error;
use std::fs::OpenOptions;
use csv::Writer;
use serde::Serialize; // Add this for serialization

// Create a struct for the sensor data
#[derive(Serialize)]
struct SensorData {
    temperature: f32,
    humidity: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut dht11 = Dht11::new(27)?; // Note this is BCM pin 27

    let response = dht11.get_reading()?;
    
    let sensor_data = SensorData {
        temperature: response.temperature,
        humidity: response.humidity,
    };

    // Define the API endpoint URL
    let api_url = "https://example.com/api/data"; // Replace with your API URL

    // Save data to a local CSV file
    let mut csv_writer = Writer::from_path("sensor_data.csv")?;
    csv_writer.serialize(sensor_data)?;

    // Create a reqwest client
    let client = Client::new();

    // Send a POST request with the JSON data
    let response = client
        .post(api_url)
        .json(&sensor_data)
        .send()?;

    if response.status().is_success() {
        println!("Data sent successfully to the API");
    } else {
        eprintln!("Failed to send data to the API: {:?}", response);
    }

    Ok(())
}
