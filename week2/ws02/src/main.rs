#![allow(dead_code)]

use std::io;
use std::error::Error;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct CSVEntry {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum TimeOfDay {
    Morning,
    Midday,
    Evening,
    Midnight,
    Total
}

#[derive(Debug)]
struct Entry {
    time_period: String,
    station: String,
    entries: HashMap<TimeOfDay, i32>,
    exits: HashMap<TimeOfDay, i32>,
    latitude: f64,
    longitude: f64,
}

fn convert_csventry_to_entry(csv_entry: &CSVEntry) -> Entry {
    let mut entry = Entry {
        time_period: csv_entry.time_period.clone(),
        station: csv_entry.station.clone(),
        entries: HashMap::new(),
        exits: HashMap::new(),
        latitude: csv_entry.latitude,
        longitude: csv_entry.longitude
    };

    if let Some(e) = csv_entry.entries_morning {
        entry.entries.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_entry.entries_midday {
        entry.entries.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_entry.entries_evening {
        entry.entries.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_entry.entries_midnight {
        entry.entries.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_entry.entries_total {
        entry.entries.insert(TimeOfDay::Total, e);
    }

    if let Some(e) = csv_entry.exits_morning {
        entry.exits.insert(TimeOfDay::Morning, e);
    }
    if let Some(e) = csv_entry.exits_midday {
        entry.exits.insert(TimeOfDay::Midday, e);
    }
    if let Some(e) = csv_entry.exits_evening {
        entry.exits.insert(TimeOfDay::Evening, e);
    }
    if let Some(e) = csv_entry.exits_midnight {
        entry.exits.insert(TimeOfDay::Midnight, e);
    }
    if let Some(e) = csv_entry.exits_total {
        entry.exits.insert(TimeOfDay::Total, e);
    }

    entry

}



/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn most_common_station(entries: Vec<Entry>) -> Vec<Entry> {
    let mut years = entries.iter()
        .map(|e| &e.time_period)
        .collect::<Vec<&String>>();
    years.sort();
    years.dedup();

    for year in years {
        let most_common_in_year = entries.iter()
            .filter(|e| e.time_period.eq(year))
            .fold(HashMap::new(), |mut map, val| {
                    map.entry(&val.station)
                        .and_modify(|frq| *frq += 1)
                        .or_insert(1);

                    map
            })
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .unwrap().0;
            println!("The most common station in year {year} was {}", most_common_in_year);
    }

    entries
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("trains.csv");

    let mut entries: Vec<CSVEntry> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;
    
    let mut _entries = entries.iter().map(|e| convert_csventry_to_entry(e)).collect::<Vec<Entry>>();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let most_common_time = _entries.iter()
        .filter(|&e| e.station.eq(&input[..input.len() - 1]))
        .fold(HashMap::new(), |mut map, val| {
            map.extend(val.entries.iter().map(|(k, v)| (k.clone(), v.clone())));
            map
        })
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .unwrap().0;
    
    Ok(())
}
