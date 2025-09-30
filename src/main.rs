extern crate clap;

use clap::{App, Arg};
mod map; 

fn main() {
    let matches = App::new("leaflet")
        .version("1.3")
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
            .default_value("#fbb4ae,#b3cde3,#ccebc5,#decbe4,#fed9a6,#ffffcc,#e5d8bd,#fddaec")
        )
        .arg(
            Arg::with_name("LAT_COL")
            .short("y")
            .long("lat-col")
            .value_name("LAT_COL")
            .help("Name of the latitude column")
            .takes_value(true)
            .default_value("latitude")
        )
        .arg(
            Arg::with_name("LNG_COL")
            .short("x")
            .long("lng-col")
            .value_name("LNG_COL")
            .help("Name of the longitude column")
            .takes_value(true)
            .default_value("longitude")
        )
        .arg(
            Arg::with_name("NAME_COL")
            .short("n")
            .long("name-col")
            .value_name("NAME_COL")
            .help("Name of the name column")
            .takes_value(true)
            .default_value("name")
        )
        .arg(
            Arg::with_name("CATEGORY_COL")
            .long("category-col")
            .short("g")
            .value_name("CATEGORY_COL")
            .help("Name of the category column")
            .takes_value(true)
            .default_value("category")
        )
        .get_matches();

    let mapping = map::ColumnMapping {
        lat_col: matches.value_of("LAT_COL").unwrap().to_string(),
        lng_col: matches.value_of("LNG_COL").unwrap().to_string(),
        name_col: matches.value_of("NAME_COL").unwrap().to_string(),
        category_col: matches.value_of("CATEGORY_COL").unwrap().to_string(),
    };

    let mut data: Vec<map::Place> = Vec::new();
    if matches.is_present("FILE") {
        let file = matches.value_of("FILE").unwrap();
        data.append(&mut map::file_to_places(&file, &mapping));
    } else {
        data.append(&mut map::stdin_to_places(&mapping));
    }

    let colors = matches.value_of("COLORS").unwrap().to_string();
    let colors = colors.split(",").map(|color| format!("'{}'", color)).collect::<Vec<String>>().join(",");
    map::render(data, &colors);
}
