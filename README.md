# Raspberry Pi Temperature and Humidity Monitoring

This Rust program reads temperature and humidity data from a DHT11 sensor connected to a Raspberry Pi's GPIO pin. The data is displayed on the console and stored in a CSV file.

## Prerequisites

- Raspberry Pi with GPIO pins
- Rust programming language installed (https://www.rust-lang.org/tools/install)
- `rppal` and `csv` crates added to your project dependencies

## Setup

1. Clone this repository to your Raspberry Pi or copy the source code to a new Rust project directory.

2. Make sure your DHT11 sensor is properly connected to a GPIO pin on your Raspberry Pi. Update the `DHT_PIN` constant in the code to match the GPIO pin number you've connected the data pin of the DHT11 sensor to.

3. Open a terminal and navigate to the project directory.

4. Compile and run the program using the following command:

   ```bash
   cargo run

## Usage

The program will start reading temperature and humidity data from the DHT11 sensor in a loop. Every 2 seconds, it will display the data on the console and store it in a CSV file named sensor_data.csv.
CSV File Format

The CSV file sensor_data.csv will have the following format:

## Output
Temperature (°C), Humidity (%)
22.5, 45.0
23.0, 46.5
...


## Dependencies
rppal crate: A Rust library for GPIO access on the Raspberry Pi.
csv crate: A Rust library for reading and writing CSV files.


Feel free to copy and paste this content into your preferred Git editor or commit message!
