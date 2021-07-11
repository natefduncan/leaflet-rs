extern crate clap;

use clap::{App, Arg};
mod map; 

fn main() {
    let matches = App::new("leaflet")
        .version("1.1")
        .author("Nate D.")
        .about("CSV of latitdue and longitudes to Leaflet HTML file.")
        .arg(
            Arg::with_name("FILE")
                .help("CSV file to use. If blank, uses stdin.")
                .required(false)
                .index(1),
        )
        .get_matches();

    let mut data: Vec<map::Place> = Vec::new();
    if matches.is_present("FILE") {
        let file = matches.value_of("FILE").unwrap();
        data.append(&mut map::file_to_places(&file));
    } else {
        data.append(&mut map::stdin_to_places());
    }
    map::render(data);
}
