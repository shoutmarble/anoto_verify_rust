use ndarray::{Array2, Array3};
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

pub fn save_as_json(bitmatrix: &Array3<i32>, base_filename: &str) -> Result<(), Box<dyn Error>> {
    let filename = format!("output/{}.json", base_filename);
    let data: Vec<Vec<Vec<i32>>> = bitmatrix.outer_iter().map(|row| row.outer_iter().map(|col| col.to_vec()).collect()).collect();
    let file = File::create(filename)?;
    serde_json::to_writer(file, &data)?;
    Ok(())
}

pub fn save_as_txt(bitmatrix: &Array3<i32>, base_filename: &str) -> Result<(), Box<dyn Error>> {
    let filename = format!("output/{}.txt", base_filename);
    let mut file = File::create(filename)?;
    for row in bitmatrix.outer_iter() {
        for col in row.outer_iter() {
            write!(file, "[{} {}] ", col[0], col[1])?;
        }
        writeln!(file)?;
    }
    Ok(())
}

pub fn load_from_json(json_path: &str) -> Result<Array2<i32>, Box<dyn Error>> {
    let file = File::open(json_path)?;
    let data: Vec<Vec<i32>> = serde_json::from_reader(file)?;
    let height = data.len();
    let width = data[0].len();
    let mut bitmatrix = Array2::zeros((height, width));
    for (i, row) in data.into_iter().enumerate() {
        for (j, val) in row.into_iter().enumerate() {
            bitmatrix[[i, j]] = val;
        }
    }
    Ok(bitmatrix)
}

pub fn load_array3_from_json(json_path: &str) -> Result<Array3<i32>, Box<dyn Error>> {
    let file = File::open(json_path)?;
    let data: Vec<Vec<Vec<i32>>> = serde_json::from_reader(file)?;
    let height = data.len();
    let width = data[0].len();
    let depth = data[0][0].len();
    let mut bitmatrix = Array3::zeros((height, width, depth));
    for (i, row) in data.into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            for (k, val) in col.into_iter().enumerate() {
                bitmatrix[[i, j, k]] = val;
            }
        }
    }
    Ok(bitmatrix)
}