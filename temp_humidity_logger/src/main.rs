use simple_dht11::dht11::Dht11;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dht11 = Dht11::new(4);

    // Open or create the CSV file for appending data
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("sensor_data.csv")?;

    loop {
        let response = dht11.get_reading();

        println!("Temperature: {}°C, Humidity: {}%", response.humidity, response.temperature);

        // Format the data for storing in csv file
        let csv_line = format!("Temperature: {}°C, Humidity: {}%\n", response.temperature, response.humidity);

        // Write data to the CSV file
        if let Err(err) = file.write_all(csv_line.as_bytes()) {
            eprintln!("Error writing data to CSV: {}", err);
        }

        // Flush the file to ensure data is written immediately
        if let Err(err) = file.flush() {
            eprintln!("Error flushing data to CSV: {}", err);
        }

        // Sleep for 1 second before the next reading
        sleep(Duration::from_secs(1));
    }
}
