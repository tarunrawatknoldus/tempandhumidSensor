use simple_dht11::dht11::Dht11;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use csv::Writer;
use csv::WriterBuilder;
use std::fs::OpenOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dht11 = Dht11::new(27); // Note this is BCM

    // Open or create the CSV file for appending data
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("sensor_data.csv")?;

    // Create a CSV writer with custom delimiter and headers
    let mut csv_writer = WriterBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_writer(file);

    loop {
        let response = dht11.get_reading();

        println!("Temperature: {}°C", response.temperature);
        println!("Humidity: {}%", response.humidity);

        // Write data to the CSV file
        csv_writer.serialize((response.temperature, response.humidity))?;

        // Sleep for 1 second before the next reading
        sleep(Duration::from_secs(1));
    }
}
