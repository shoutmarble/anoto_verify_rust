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
                .help("Generate Anoto dot pattern with optional shape and section: x1 y1 x2 y2 (defaults: 9 16 10 2)")
                .num_args(0..=4)
                .value_names(["x1", "y1", "x2", "y2"]),
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

    let mut x1 = 9;
    let mut y1 = 16;
    let mut x2 = 10;
    let mut y2 = 2;

    if let Some(values) = matches.get_many::<String>("generate") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        x1 = v.get(0).unwrap_or(&"9".to_string()).parse().unwrap_or(9);
        y1 = v.get(1).unwrap_or(&"16".to_string()).parse().unwrap_or(16);
        x2 = v.get(2).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
        y2 = v.get(3).unwrap_or(&"2".to_string()).parse().unwrap_or(2);
    }

    anoto_dots::gen_matrix(x1, y1, x2, y2)?;

    if let Some(values) = matches.get_many::<String>("generate_json") {
        let v: Vec<String> = values.map(|s| s.to_string()).collect();
        let filename = v.get(0).unwrap_or(&"plot_data.json".to_string()).clone();
        let x2: i32 = v.get(1).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
        let y2: i32 = v.get(2).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
        anoto_dots::gen_matrix_from_json(&filename, x2, y2)?;
    }

    Ok(())
}