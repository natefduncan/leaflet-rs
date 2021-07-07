extern crate clap;

use clap::{App, Arg};
use tera::Tera;
mod lib;

fn main() {
    let matches = App::new("leaflet")
        .version("1.0")
        .author("Nate D.")
        .about("CSV of latitdue and longitudes to Leaflet HTML file.")
        .arg(
            Arg::with_name("FILE")
                .help("CSV file to use. If blank, uses stdin.")
                .required(false)
                .index(1),
        )
        .get_matches();

    let mut data: Vec<lib::Place> = Vec::new();
    if matches.is_present("FILE") {
        let file = matches.value_of("FILE").unwrap();
        data.append(&mut lib::file_to_places(&file));
    } else {
        data.append(&mut lib::stdin_to_places());
    }
    let templates = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    lib::render(templates, data);
}
