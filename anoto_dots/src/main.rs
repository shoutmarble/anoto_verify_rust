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
                .value_names(["x1", "y1", "x2", "y2"])
                .default_values(["9", "16", "10", "2"]),
        );

    let matches = app.get_matches();

    if matches.contains_id("generate") {
        let values: Vec<String> = matches.get_many::<String>("generate")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect();
        
        let x1: usize = values.first().unwrap_or(&"9".to_string()).parse().unwrap_or(9);
        let y1: usize = values.get(1).unwrap_or(&"16".to_string()).parse().unwrap_or(16);
        let x2: i32 = values.get(2).unwrap_or(&"10".to_string()).parse().unwrap_or(10);
        let y2: i32 = values.get(3).unwrap_or(&"2".to_string()).parse().unwrap_or(2);

        anoto_dots::gen_matrix(x1, y1, x2, y2)?;
    }

    Ok(())
}
