# Leaflet Rust

CSV file or stdin to complete leaflet HTML. Can be used with [goose](https://github.com/natefduncan/goose-rs.git) to visualize POI. 

## Installation 

`cargo install --path .`

## Usage

`goose "Restaurants" "Mesa, AZ" -f csv | leaflet > map.html && open map.html`
