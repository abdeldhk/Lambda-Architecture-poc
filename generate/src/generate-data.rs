use rand::{distributions::Alphanumeric, seq::SliceRandom, Rng};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct Aircraft {
    model: String,
    registration: String,
}

#[derive(Serialize)]
struct FlightEvent {
    flight_id: String,
    airline: String,
    origin: String,
    destination: String,
    scheduled_departure: String,
    actual_departure: String,
    status: String,
    aircraft: Aircraft,
}

fn get_iso_timestamp(offset_minutes: i64) -> String {
    let now = chrono::Utc::now() + chrono::Duration::minutes(offset_minutes);
    now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn generate_flight_event() -> FlightEvent {
    let airlines = ["Emirates", "SWISS", "Qatar Airways", "Lufthansa", "Turkish Airlines"];
    let aircraft_models = ["Boeing 777", "Airbus A330", "Boeing 787", "Airbus A350"];
    let airports = ["DXB", "ZRH", "JFK", "IST", "DOH", "LHR", "CDG", "FRA", "AMS", "SIN"];

    let mut rng = rand::thread_rng();
    let airline = airlines.choose(&mut rng).unwrap().to_string();
    let origin = airports.choose(&mut rng).unwrap().to_string();
    let mut destination = airports.choose(&mut rng).unwrap().to_string();
    while destination == origin {
        destination = airports.choose(&mut rng).unwrap().to_string();
    }

    let flight_id = format!("{}{}", &airline[..2].to_uppercase(), rng.gen_range(100..999));
    let scheduled_departure = get_iso_timestamp(rng.gen_range(0..10000));
    let actual_departure = get_iso_timestamp(rng.gen_range(0..10000) + rng.gen_range(-10..30));

    FlightEvent {
        flight_id,
        airline,
        origin,
        destination,
        scheduled_departure,
        actual_departure,
        status: "departed".to_string(),
        aircraft: Aircraft {
            model: aircraft_models.choose(&mut rng).unwrap().to_string(),
            registration: format!("A6-E{}", rng.sample(Alphanumeric) as char),
        },
    }
}

fn write_flights_to_file(path: &str, count: usize) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for _ in 0..count {
        let event = generate_flight_event();
        let json = serde_json::to_string(&event).unwrap();
        writeln!(writer, "{}", json)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("data/flights")?;
    write_flights_to_file("data/flights/flights_big_1.json", 500)?;
    write_flights_to_file("data/flights/flights_big_2.json", 500)?;
    println!("Generated 2 files in data/flights/");
    Ok(())
}
