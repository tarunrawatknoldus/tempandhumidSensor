use simple_dht11::dht11::Dht11;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use tokio::fs::File;
use tokio::sync::mpsc;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

#[derive(Debug, serde::Serialize)]
struct SensorReading {
    temperature: f32,
    humidity: f32,
}

async fn record_sensor_data() -> Result<(), Box<dyn Error>> {
    let mut dht11 = Dht11::new(4);

    // Open or create the CSV file for appending data
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("sensor_data.csv")?;

    let (tx, mut rx) = mpsc::channel::<SensorReading>(32);

    // Spawn a background task to continuously record sensor data
    tokio::spawn(async move {
        loop {
            let response = dht11.get_reading();

            let reading = SensorReading {
                temperature: response.temperature,
                humidity: response.humidity,
            };

            // Send the reading to the main task for storage
            if let Err(_) = tx.send(reading).await {
                break;
            }

            // Sleep for 1 second before the next reading
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    while let Some(reading) = rx.recv().await {
        println!("Temperature: {}°C, Humidity: {}%", reading.humidity, reading.temperature);

        // Format the data for storing in the CSV file
        let csv_line = format!("Temperature: {}°C, Humidity: {}%\n", reading.temperature, reading.humidity);

        // Write data to the CSV file
        if let Err(err) = tokio::fs::write("sensor_data.csv", csv_line).await {
            eprintln!("Error writing data to CSV: {}", err);
        }
    }

    Ok(())
}

async fn get_latest_sensor_data() -> impl Responder {
    // In a real-world application, you would read the latest data from your CSV file.
    // For simplicity, we'll return dummy data here.
    let dummy_data = SensorReading {
        temperature: 25.5,
        humidity: 50.0,
    };

    HttpResponse::Ok().json(dummy_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";

    // Spawn a background task to continuously record sensor data
    tokio::spawn(record_sensor_data());

    // Start the Actix web server
    HttpServer::new(|| {
        App::new()
            .route("/api/latest", web::get().to(get_latest_sensor_data))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
