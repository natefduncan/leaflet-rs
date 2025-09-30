extern crate tera;
use csv;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{self, Write, Read};
use std::fs; 
use tera::{Context, Tera};

#[derive(Debug, Deserialize, Serialize)]
pub struct Place {
    latitude: f64,
    longitude: f64,
    name : String, 
    category : Option<String>
}

pub fn render(places: Vec<Place>, colors : &str) {
    let center: [f64; 2] = average_coords(&places);
    let mut context = Context::new();
    context.insert("center_lat", &center[0]);
    context.insert("center_lng", &center[1]);
    let data = vector_to_string(places);
    context.insert("data", &data);
    context.insert("colors", &colors); 
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
    let output : String = data.iter().map(|place| serde_json::to_string(&place).unwrap()).collect::<Vec<String>>().join(","); 
    output
}

pub struct ColumnMapping {
    pub lat_col: String,
    pub lng_col: String,
    pub name_col: String,
    pub category_col: String,
}

fn reader_to_places<R: Read>(rdr : &mut csv::Reader<R>, mapping: &ColumnMapping) -> Vec<Place> {
    let mut output: Vec<Place> = Vec::new();
    let headers = rdr.headers().expect("Could not read headers").clone();

    let lat_idx = headers.iter().position(|h| h == mapping.lat_col)
        .expect(&format!("Could not find column '{}'", mapping.lat_col));
    let lng_idx = headers.iter().position(|h| h == mapping.lng_col)
        .expect(&format!("Could not find column '{}'", mapping.lng_col));
    let name_idx = headers.iter().position(|h| h == mapping.name_col)
        .expect(&format!("Could not find column '{}'", mapping.name_col));
    let category_idx = headers.iter().position(|h| h == mapping.category_col);

    for result in rdr.records() {
        let record = result.expect("Could not read record");

        let latitude: f64 = record.get(lat_idx)
            .expect("Missing latitude value")
            .parse()
            .expect("Could not parse latitude as number");
        let longitude: f64 = record.get(lng_idx)
            .expect("Missing longitude value")
            .parse()
            .expect("Could not parse longitude as number");
        let name = record.get(name_idx)
            .expect("Missing name value")
            .to_string();
        let category = category_idx
            .and_then(|idx| record.get(idx))
            .map(|s| s.to_string())
            .or(Some("Place".to_string()));

        output.push(Place { latitude, longitude, name, category });
    }
    return output;
}

pub fn stdin_to_places(mapping: &ColumnMapping) -> Vec<Place> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    reader_to_places::<io::Stdin>(&mut rdr, mapping)
}

pub fn file_to_places(file_path: &str, mapping: &ColumnMapping) -> Vec<Place> {
    let path = std::fs::canonicalize(file_path).expect("Could not get path.");
    let mut rdr = csv::Reader::from_path(path).expect("Could not get reader.");
    reader_to_places::<fs::File>(&mut rdr, mapping)
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
        var data = [{{ data }}];

        //Tile Layer
        var tile_layer = L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
            maxZoom: 18,
            id: "osm.standard"
        }); 

        //Color palette
        var colors = [{{colors}}]; 
        layers = {}; 

        //Create markers and add to layers
        data.forEach(function (value, idx) {
            if (!(value.category in layers)) {
                layers[value.category] = [];
            }
            var color_idx = Object.keys(layers).indexOf(value.category);
            if (color_idx == -1) {
                var color = "black"; 
            } else {
                var color = colors[color_idx];
            }
            var temp_marker = L.circle([value.latitude, value.longitude], {
                color: color,
                fillColor: color,
                fillOpacity: 0.5,
                radius: 250
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
