extern crate clap;

use clap::{App, Arg};
mod map; 

fn main() {
    let matches = App::new("leaflet")
        .version("1.2")
        .author("Nate D.")
        .about("CSV of latitdue and longitudes to Leaflet HTML file.")
        .arg(
            Arg::with_name("FILE")
                .help("CSV file to use. If blank, uses stdin.")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("COLORS")
            .short("c")
            .long("colors")
            .value_name("COLORS")
            .help("List of colors to use for markers.")
            .takes_value(true)
        )
        .get_matches();

    let mut data: Vec<map::Place> = Vec::new();
    if matches.is_present("FILE") {
        let file = matches.value_of("FILE").unwrap();
        data.append(&mut map::file_to_places(&file));
    } else {
        data.append(&mut map::stdin_to_places());
    }

    let colors = matches.value_of("COLORS").unwrap_or("#fbb4ae,#b3cde3,#ccebc5,#decbe4,#fed9a6,#ffffcc,#e5d8bd,#fddaec"); 
    let colors = colors.split(",").map(|color| format!("'{}'", color)).collect::<Vec<String>>().join(","); 
    map::render(data, &colors);
}
