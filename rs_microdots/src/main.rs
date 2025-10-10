use clap::{Arg, Command};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    let app = Command::new("anoto_dots")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Generates and verifies Anoto dot patterns")
        .arg(
            Arg::new("generate")
                .short('g')
                .long("generate")
                .help("Generate Anoto dot pattern with shape and section: height width sect_u sect_v (defaults: 9 16 10 2)")
                .num_args(0..=4)
                .value_names(["height", "width", "sect_u", "sect_v"]),
        )
        .arg(
            Arg::new("generate_json")
                .short('j')
                .long("generate-json")
                .help("Generate from JSON file: filename")
                .num_args(1)
                .value_names(["filename"]),
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .help("Decode position from 6x6 section file: filename")
                .num_args(1)
                .value_names(["filename"]),
        )
        .arg(
            Arg::new("position")
                .short('p')
                .long("pos")
                .help("Extract 6x6 section at position: row col")
                .num_args(2)
                .value_names(["row", "col"]),
        );

    let matches = app.get_matches();

    // Get position if specified
    let position = if let Some(values) = matches.get_many::<String>("position") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        let row = v.first().unwrap().parse().unwrap_or(0);
        let col = v.get(1).unwrap().parse().unwrap_or(0);
        Some((row, col))
    } else {
        None
    };

    if let Some(values) = matches.get_many::<String>("generate") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        let height = v.first().unwrap_or(&"9".to_string()).parse().unwrap_or(9);
        let width = v.get(1).unwrap_or(&"16".to_string()).parse().unwrap_or(16);
        let sect_u = v.get(2).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
        let sect_v = v.get(3).unwrap_or(&"2".to_string()).parse().unwrap_or(2);

        let bitmatrix = anoto_dots::generate_matrix_only(height, width, sect_u, sect_v)?;
        
        // If position is specified, extract 6x6 section
        if let Some(pos) = position {
            anoto_dots::extract_6x6_section(&bitmatrix, pos)?;
        }
        
        // Save the full matrix
        anoto_dots::save_generated_matrix(&bitmatrix, height, width, sect_u, sect_v)?;
    }

    if let Some(values) = matches.get_many::<String>("generate_json") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        let filename = v.first().unwrap_or(&"plot_data.json".to_string()).clone();
        
        let bitmatrix = anoto_dots::load_matrix_from_json(&filename)?;
        
        // If position is specified, extract 6x6 section
        if let Some(pos) = position {
            anoto_dots::extract_6x6_section(&bitmatrix, pos)?;
        }
        
        // Save the full matrix
        anoto_dots::save_matrix_from_json(&bitmatrix, &filename)?;
    }

    if let Some(values) = matches.get_many::<String>("decode") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        let filename = v.first().unwrap().clone();
        
        let section = anoto_dots::persist_json::load_6x6_section(&filename)?;
        match anoto_dots::decode_utils::decode_position(&section) {
            Some((row, col)) => println!("POS ({}, {})", row, col),
            None => println!("Could not decode position from section"),
        }
    }

    Ok(())
}