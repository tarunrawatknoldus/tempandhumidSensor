use simple_dht11::dht11::Dht11;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut dht11 = Dht11::new(27); // Note this is BCM

    loop {
        let response = dht11.get_reading();

        println!("Temperature: {}°C", response.temperature);
        println!("Humidity: {}%", response.humidity);

        // Sleep for 1 second before the next reading
        sleep(Duration::from_secs(1));
    }
}
