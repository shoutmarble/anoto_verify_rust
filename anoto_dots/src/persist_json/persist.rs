use ndarray::Array3;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn save_bitmatrix_text(bitmatrix: &Array3<i8>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    for row in 0..bitmatrix.dim().0 {
        for col in 0..bitmatrix.dim().1 {
            let x_bit = bitmatrix[[row, col, 0]];
            let y_bit = bitmatrix[[row, col, 1]];
            write!(file, "[{} {}]", x_bit, y_bit)?;
            if col < bitmatrix.dim().1 - 1 {
                write!(file, " ")?;
            }
        }
        writeln!(file)?;
    }
    Ok(())
}

pub fn save_bitmatrix_json(bitmatrix: &Array3<i8>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut data = Vec::new();
    for row in 0..bitmatrix.dim().0 {
        let mut row_data = Vec::new();
        for col in 0..bitmatrix.dim().1 {
            let pair = vec![bitmatrix[[row, col, 0]], bitmatrix[[row, col, 1]]];
            row_data.push(pair);
        }
        data.push(row_data);
    }
    let file = File::create(filename)?;
    serde_json::to_writer(file, &data)?;
    Ok(())
}