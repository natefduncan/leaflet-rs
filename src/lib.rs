extern crate tera;
use csv;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::io::{self, Write};
use tera::{Context, Tera};

#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    latitude: f64,
    longitude: f64,
    name : String
}

pub fn render(templates: &mut tera::Tera, places: Vec<Place>) {
    templates.autoescape_on(vec![]);
    let center: [f64; 2] = average_coords(&places);
    let mut context = Context::new();
    context.insert("center_lat", &center[0]);
    context.insert("center_lng", &center[1]);
    let data = vector_to_string(places);
    context.insert("data", &data);
    match templates.render("map.html", &context) {
        Ok(s) => io::stdout().write(s.as_bytes()),
        Err(e) => {
            io::stdout().write("Error: ".as_bytes())
        }
    }.unwrap();
    ()
}

fn average_coords(places: &Vec<Place>) -> [f64; 2] {
    let mut lat: f64 = 0.0;
    let mut lng: f64 = 0.0;
    for place in places {
        lat = lat + place.latitude;
        lng = lng + place.longitude;
    }
    return [lat / places.len() as f64, lng / places.len() as f64];
}

pub fn vector_to_string(data: Vec<Place>) -> String {
    let mut output = String::new();
    output.push_str("[");
    let mut is_first: u32 = 1;
    for place in data {
        let string = serde_json::to_string(&place).unwrap();
        if is_first == 1 {
            is_first = 0;
        } else {
            output.push_str(",");
        }
        output.push_str(&string);
    }
    output.push_str("]");
    output
}

pub fn stdin_to_places() -> Vec<Place> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut output: Vec<Place> = Vec::new();
    for result in rdr.deserialize() {
        let record: Place = result.expect("Could not coerce to places.");
        output.push(record);
    }
    return output;
}

pub fn file_to_places(file_path: &str) -> Vec<Place> {
    let mut rdr = csv::Reader::from_path(&file_path).expect("Could not get from path.");
    let mut output: Vec<Place> = Vec::new();
    for result in rdr.deserialize() {
        let record: Place = result.expect("Could not coerce to places.");
        output.push(record);
    }
    return output;
}
