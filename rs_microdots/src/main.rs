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
                .help("Generate from JSON file: filename [x2 y2] (defaults: x2=10, y2=10)")
                .num_args(1..=3)
                .value_names(["filename", "x2", "y2"]),
        );

    let matches = app.get_matches();

    if let Some(values) = matches.get_many::<String>("generate") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        let height = v.first().unwrap_or(&"9".to_string()).parse().unwrap_or(9);
        let width = v.get(1).unwrap_or(&"16".to_string()).parse().unwrap_or(16);
        let sect_u = v.get(2).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
        let sect_v = v.get(3).unwrap_or(&"2".to_string()).parse().unwrap_or(2);

        anoto_dots::gen_matrix(height, width, sect_u, sect_v)?;
        // gen_pdf::gen_all_dots_anoto_pdf()?;
    }

    if let Some(values) = matches.get_many::<String>("generate_json") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        let filename = v.first().unwrap_or(&"plot_data.json".to_string()).clone();
        anoto_dots::gen_matrix_from_json(&filename)?;
        // gen_pdf::gen_all_dots_anoto_pdf()?;
    }

    Ok(())
}