use std::{time::Duration, thread};
use chrono::{Utc, Duration as ChronoDuration};
use rand::{seq::SliceRandom, Rng};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::json;


// This code simulates the ingestion of flight events into a Kafka topic named "flight-events".
// It generates random flight data including airline, flight number, origin, destination, status, and aircraft details.
// The events are produced continuously with a 1-second interval between each event.

fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Failed to create Kafka producer");

    let airlines = vec!["Emirates", "Qatar Airways", "Lufthansa", "Air France"];
    let aircraft_models = vec!["Boeing 777", "Airbus A380", "Boeing 787", "Airbus A320"];
    let origins = vec!["DXB", "CDG", "JFK", "DOH"];
    let destinations = vec!["ZRH", "FRA", "LHR", "MAD"];
    let statuses = vec!["scheduled", "boarding", "departed", "delayed", "cancelled"];

    loop {
        let airline = airlines.choose(&mut rand::thread_rng()).unwrap();
        let flight_number = rand::thread_rng().gen_range(100..999);
        let flight_id = format!("{}{}", &airline[0..2].to_uppercase(), flight_number);

        let origin = origins.choose(&mut rand::thread_rng()).unwrap();
        let destination = destinations.choose(&mut rand::thread_rng()).unwrap();
        let status = statuses.choose(&mut rand::thread_rng()).unwrap();

        let scheduled_departure = Utc::now() + ChronoDuration::minutes(rand::thread_rng().gen_range(10..180));
        let actual_departure = if status == &"departed" {
            Some(scheduled_departure + ChronoDuration::minutes(rand::thread_rng().gen_range(1..10)))
        } else {
            None
        };

        let aircraft_model = aircraft_models.choose(&mut rand::thread_rng()).unwrap();
        let registration = format!("A6-{}", ('A'..='Z').choose(&mut rand::thread_rng()).unwrap());

        let event = json!({
            "flight_id": flight_id,
            "airline": airline,
            "origin": origin,
            "destination": destination,
            "scheduled_departure": scheduled_departure.to_rfc3339(),
            "actual_departure": actual_departure.map(|t| t.to_rfc3339()),
            "status": status,
            "aircraft": {
                "model": aircraft_model,
                "registration": registration
            }
        });

        let payload = event.to_string();

        let record = FutureRecord::to("flight-events")
            .payload(&payload)
            .key(&flight_id);

        let _ = producer.send(record, Duration::from_secs(0));

        println!("Produced flight event: {}", payload);
        thread::sleep(Duration::from_millis(1000));
    }
}    