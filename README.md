# DHT11 Sensor Data Logger

This Rust program reads temperature and humidity data from a DHT11 sensor and logs it to a CSV (Comma-Separated Values) file. The program uses the [simple_dht11](https://crates.io/crates/simple_dht11) crate to interface with the DHT11 sensor and the [csv](https://crates.io/crates/csv) crate for CSV file handling.

## Prerequisites

Before running this program, ensure that you have the following prerequisites installed:

- Rust: You can download and install Rust from the official website [here](https://www.rust-lang.org/tools/install).

## Usage

1. Clone the repository to your local machine:

   ```shell
   git clone https://github.com/your-username/dht11-sensor-logger.git
   cd dht11-sensor-logger
2. Build and run the program:
   Cargo run
   The program will continuously read data from the DHT11 sensor, print it to the console, and log it to a CSV file named sensor_data.csv.
   To stop the program, press Ctrl + C in the terminal where it's running.
3. You can configure the GPIO pin to which the DHT11 sensor is connected by modifying the following line in the main function:
   let mut dht11 = Dht11::new(4); // Replace 4 with the GPIO pin number you are using.

##Logging

    The program logs the temperature and humidity data to a CSV file (sensor_data.csv) in the current working directory.
    Each line in the CSV file contains two values: temperature in degrees Celsius and humidity in percentage, separated by a comma.

   
