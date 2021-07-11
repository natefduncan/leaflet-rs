extern crate tera;
use csv;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{self, Write};
use tera::{Context, Tera};

#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    latitude: f64,
    longitude: f64,
    name : String, 
    category : String
}

pub fn render(places: Vec<Place>) {
    let center: [f64; 2] = average_coords(&places);
    let mut context = Context::new();
    context.insert("center_lat", &center[0]);
    context.insert("center_lng", &center[1]);
    let data = vector_to_string(places);
    context.insert("data", &data);
    match Tera::one_off(TEMPLATE, &context, false) {
        Ok(s) => io::stdout().write(s.as_bytes()), 
        Err(e) => {
            eprintln!("{}", e); 
            io::stdout().write("Error".as_bytes())
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
    let path = std::fs::canonicalize(file_path).expect("Could not get path."); 
    let mut rdr = csv::Reader::from_path(path).expect("Could not get reader.");
    let mut output: Vec<Place> = Vec::new();
    for result in rdr.deserialize() {
        let record: Place = result.expect("Could not coerce to places.");
        output.push(record);
    }
    return output;
}

pub const TEMPLATE : &str = r#"
<!DOCTYPE html>
<html>

<head>
    <title>Leaflet</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.7.1/dist/leaflet.css"
        integrity="sha512-xodZBNTC5n17Xt2atTPuE1HxjVMSvLVW9ocqUKLsCC5CXdbqCmblAshOMAS6/keqq/sMZMZ19scR4PsZChSR7A=="
        crossorigin="" />
    <script src="https://unpkg.com/leaflet@1.7.1/dist/leaflet.js"
        integrity="sha512-XQoYMqMTK8LvdxXYG3nZ448hOEQiglfqkJs1NOQV44cWnUrBc8PkAOcXy20w0vlaXaVUearIOBhiXZ5V3ynxwA=="
        crossorigin=""></script>
    <style>
        body {
            margin: 0;
            height: 100vh;
            display: flex;
            flex-direction: column;
        }

        #map {
            flex-grow: 1;
            width: 100%;
        }
    </style>
</head>

<body>
    <div id="map"></div>
    <script>
        //Data
        var data = {{ data }};

        //Tile Layer
        var tile_layer = L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
            maxZoom: 18,
            id: "osm.standard"
        }); 

        //Color palette
        var colors = ['#fff100', '#ff8c00', '#e81123','#ec008c','#68217a','#00188f','#00bcf2','#00b294','#009e49','#bad80a']; 
        layers = {}; 

        //Create markers and add to layers
        data.forEach(function (value, idx) {
            if (!(value.category in layers)) {
                layers[value.category] = [];
            }
            var color_idx = Object.keys(layers).indexOf(value.category);
            var color = colors[color_idx];
            var temp_marker = L.circle([value.latitude, value.longitude], {
                color: color,
                fillColor: color,
                fillOpacity: 0.5,
                radius: 500
            })
            .bindPopup(value.name)
            layers[value.category].push(temp_marker);  
        });

        //Create layer groups
        var layer_groups = {}; 
        Object.keys(layers).forEach(function(value, idx) {
            layer_groups[value] = L.layerGroup(layers[value]); 
        })

        var map = L.map("map", {
            center: [{{ center_lat }}, {{ center_lng }}], 
            zoom: 13, 
            layers : [tile_layer]
        });

        var baseMaps = {
            "Map": tile_layer,
        };

        L.control.layers(baseMaps, layer_groups).addTo(map);
    </script>
</body>

</html>
"#; 