use simple_dht11::dht11::Dht11;
use std::fs::File;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use csv::Writer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dht11 = Dht11::new(4)?; // Note this is BCM pin 27
    let mut writer = Writer::from_path("sensor_data.csv")?; // Create a CSV file

    writer.write_record(&["Timestamp", "Temperature (°C)", "Humidity (%)"])?;

    loop {
        match dht11.get_reading() {
            Ok(response) => {
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();

                println!("Temperature: {}°C", response.temperature);
                println!("Humidity: {}%", response.humidity);

                // Write data to CSV file
                writer.write_record(&[timestamp.to_string(), response.temperature.to_string(), response.humidity.to_string()])?;
                writer.flush()?;
            }
            Err(e) => {
                eprintln!("Error reading DHT11: {:?}", e);
            }
        }

        // Delay for 1 second
        sleep(Duration::from_secs(1));
    }
}
