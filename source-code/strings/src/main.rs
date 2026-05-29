// Application to parse and aggregate a data file of the following format:
// ```
// time: 2023-06-01T12:00:00Z
// temperature: 42.3
// pressure: 1013.25
// ----
// time: 2023-06-01T12:01:00Z
// temperature: 42.5
// pressure: 1013.30
// ----
// ...
// ```
//
// The data is read by the application, parsed, and aggregated to calculate the average temperature
// and pressure over the entire dataset.    As the file can be very large, the application reads
// the file line by line, processes each record, and updates the aggregate values without loading
// the entire file into memory. Additionally, it prints the number of days covered by the data,
// which is calculated based on the timestamps of the records.

use chrono::{DateTime, Utc};
use clap::Parser;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[command(version, about = "Parse data file and aggregate data")]
struct Args {
    /// The path to the data file
    #[arg(short, long)]
    file: String,
}

struct Record {
    time: DateTime<Utc>,
    temperature: f64,
    pressure: f64,
}

struct DataAggregator {
    total_temperature: f64,
    total_pressure: f64,
    count: usize,
    first_time: Option<DateTime<Utc>>,
    last_time: Option<DateTime<Utc>>,
}

impl DataAggregator {
    fn new() -> Self {
        Self {
            total_temperature: 0.0,
            total_pressure: 0.0,
            count: 0,
            first_time: None,
            last_time: None,
        }
    }

    fn add_record(&mut self, record: Record) {
        self.total_temperature += record.temperature;
        self.total_pressure += record.pressure;
        self.count += 1;
        match (self.first_time, self.last_time) {
            (None, None) => {
                self.first_time = Some(record.time);
                self.last_time = Some(record.time);
            }
            (Some(first), Some(last)) => {
                if record.time < first {
                    self.first_time = Some(record.time);
                }
                if record.time > last {
                    self.last_time = Some(record.time);
                }
            }
            _ => {
                unreachable!("first_time and last_time should be updated together");
            }
        }
    }

    fn average_temperature(&self) -> Option<f64> {
        if self.count > 0 {
            Some(self.total_temperature / self.count as f64)
        } else {
            None
        }
    }

    fn average_pressure(&self) -> Option<f64> {
        if self.count > 0 {
            Some(self.total_pressure / self.count as f64)
        } else {
            None
        }
    }

    fn days_covered(&self) -> Option<f64> {
        if let (Some(first), Some(last)) = (self.first_time, self.last_time) {
            let duration = last - first;
            Some(duration.num_seconds() as f64 / 86400.0)
        } else {
            None
        }
    }
}

fn parse_record(record_str: &str) -> Result<Record, String> {
    let mut time: Option<DateTime<Utc>> = None;
    let mut temperature: Option<f64> = None;
    let mut pressure: Option<f64> = None;
    for line in record_str.lines() {
        if line.starts_with("time:") {
            if let Some(time_str) = line.split_once(':').map(|(_, value)| value) {
                if let Ok(parsed_time) = time_str.trim().parse::<DateTime<Utc>>() {
                    time = Some(parsed_time);
                } else {
                    return Err(format!("Failed to parse time: {}", time_str.trim()));
                }
            } else {
                return Err("Missing time value".to_string());
            }
        } else if line.starts_with("temperature:") {
            if let Some(temp_str) = line.split(':').nth(1) {
                if let Ok(temperature_val) = temp_str.trim().parse::<f64>() {
                    temperature = Some(temperature_val);
                } else {
                    return Err(format!("Failed to parse temperature: {}", temp_str.trim()));
                }
            } else {
                return Err("Missing temperature value".to_string());
            }
        } else if line.starts_with("pressure:") {
            if let Some(pressure_str) = line.split(':').nth(1) {
                if let Ok(pressure_val) = pressure_str.trim().parse::<f64>() {
                    pressure = Some(pressure_val);
                } else {
                    return Err(format!("Failed to parse pressure: {}", pressure_str.trim()));
                }
            } else {
                return Err("Missing pressure value".to_string());
            }
        }
    }
    let time = time.ok_or_else(|| "Missing time field".to_string())?;
    let temperature = temperature.ok_or_else(|| "Missing temperature field".to_string())?;
    let pressure = pressure.ok_or_else(|| "Missing pressure field".to_string())?;

    Ok(Record {
        time,
        temperature,
        pressure,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut aggregator = DataAggregator::new();
    let mut record_buffer = String::new();
    let file = std::fs::File::open(&args.file).expect("Failed to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.starts_with("#") || line.trim().is_empty() {
            continue;
        }
        if line.trim() == "----" {
            if record_buffer.trim().is_empty() {
                continue;
            }
            match parse_record(&record_buffer) {
                Ok(record) => aggregator.add_record(record),
                Err(err) => {
                    eprintln!("Failed to parse record:\n{}", record_buffer);
                    eprintln!("Error: {}", err);
                }
            }
            record_buffer.clear();
            continue;
        }
        record_buffer.push_str(&line);
        record_buffer.push('\n');
    }
    if !record_buffer.trim().is_empty() {
        match parse_record(&record_buffer) {
            Ok(record) => aggregator.add_record(record),
            Err(err) => {
                eprintln!("Failed to parse record:\n{}", record_buffer);
                eprintln!("Error: {}", err);
            }
        }
    }
    match (
        aggregator.average_temperature(),
        aggregator.average_pressure(),
        aggregator.days_covered()
    ) {
        (Some(avg_temperature), Some(avg_pressure), Some(days)) => {
            println!("Average Temperature: {:.2}", avg_temperature);
            println!("Average Pressure: {:.2}", avg_pressure);
            println!("Days Covered: {:.2}", days);
            Ok(())
        }
        _ => {
            Err("No valid records found".into())
        }
    }
}
