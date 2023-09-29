use simple_dht11::dht11::Dht11;
use std::fs::File;
use std::error::Error;
use csv::Writer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dht11 = Dht11::new(27)?; // Note this is BCM pin 27
    let mut writer = Writer::from_path("sensor_data.csv")?; // Create a CSV file

    writer.write_record(&["Temperature (°C)", "Humidity (%)"])?;

    loop {
        match dht11.get_reading() {
            Ok(response) => {
                match response {
                    Ok(data) => {
                        println!("Temperature: {}°C", data.temperature);
                        println!("Humidity: {}%", data.humidity);

                        // Write data to CSV file
                        writer.write_record(&[data.temperature.to_string(), data.humidity.to_string()])?;
                        writer.flush()?;
                    }
                    Err(e) => {
                        eprintln!("Error reading DHT11: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading DHT11: {:?}", e);
            }
        }

        // Delay for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
