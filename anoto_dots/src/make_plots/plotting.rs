use plotters::prelude::*;
use std::error::Error;

// Drawing function using plotters
pub fn draw_dots(
    bitmatrix: &ndarray::Array3<i8>,
    _grid_size: f64) -> Result<(), Box<dyn Error>> {
    // Persist the bitmatrix
    crate::persist_json::save_bitmatrix_text(bitmatrix, "bitmatrix.txt")?;
    crate::persist_json::save_bitmatrix_json(bitmatrix, "bitmatrix.json")?;

    let filename = "anoto_dots.png";
    draw_dots_y_axis(bitmatrix, _grid_size)?;

    let root_area = BitMapBackend::new(filename, (800, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let num_rows = bitmatrix.dim().0 as i32;
    let num_cols = bitmatrix.dim().1 as i32;

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Anoto Dots", ("sans-serif", 40))
        .build_cartesian_2d(-10_i32..(num_cols * 10), -10_i32..(num_rows * 10))
        .unwrap();

   ctx.configure_mesh()
        .x_labels(num_cols as usize + 1)
        .x_label_formatter(&|v| format!("{}", (v / 10) ))
        .y_labels(num_rows as usize + 1)
        .y_label_formatter(&|v| format!("{}", (v / 10) ))
        .draw().unwrap();

    // Draw circles based on bitmatrix values
    // Draw circles based on bitmatrix values
    ctx.draw_series(
        (0..bitmatrix.dim().0).flat_map(|y| {
            (0..bitmatrix.dim().1).map(move |x| {
                let x_bit = bitmatrix[[y, x, 0]] as usize;
                let y_bit = bitmatrix[[y, x, 1]] as usize;
                let dot_type = x_bit + (y_bit << 1);
                let color = match dot_type {
                    0 => &BLACK, // UP
                    1 => &BLUE,  // LEFT
                    2 => &RED,   // RIGHT
                    3 => &GREEN, // DOWN
                    _ => &BLACK,
                };
                let mut x_x :i32 = x.clone() as i32;
                let mut y_y :i32 = bitmatrix.dim().0 as i32 - 1 - y as i32;
                match dot_type {
                    0 => { // UP
                        x_x = x_x * 10;
                        y_y = y_y * 10 + 2;
                    }
                    1 => { // LEFT
                        x_x = (x_x * 10) - 2;
                        y_y = y_y * 10;
                    },
                    2 => { // RIGHT
                        x_x = x_x * 10 + 2;
                        y_y = y_y * 10;
                    }
                    3 => { // DOWN
                        x_x = x_x * 10;
                        y_y = (y_y * 10) - 2;
                    },
                    _ => {}
                };

                Circle::new((x_x as i32, y_y as i32), 5, color.filled())
            })
        })
    ).unwrap();

    Ok(())
}


// Drawing function using plotters
pub fn draw_dots_y_axis(
    bitmatrix: &ndarray::Array3<i8>,
    _grid_size: f64) -> Result<(), Box<dyn Error>> {
    // Persist the bitmatrix
    crate::persist_json::save_bitmatrix_text(bitmatrix, "bitmatrix.txt")?;
    crate::persist_json::save_bitmatrix_json(bitmatrix, "bitmatrix.json")?;
    let filename = "anoto_dots_Y.png";
    
    let root_area = BitMapBackend::new(&filename, (800, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    println!("Bitmatrix size: {:?} (rows: {}, cols: {}, depth: {}, total elements: {})", 
             bitmatrix.dim(), // tuple of (row, col, depth)
             bitmatrix.dim().0, // rows
             bitmatrix.dim().1, // cols
             bitmatrix.dim().2,  // depth
             bitmatrix.len());  // num elements

    let num_rows = bitmatrix.dim().0 as i32;
    let num_cols = bitmatrix.dim().1 as i32;

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Anoto Dots", ("sans-serif", 40))
        .build_cartesian_2d(-10_i32..(num_cols * 10), (num_rows * 10)..(-10_i32))
        .unwrap();

   ctx.configure_mesh()
        .x_labels(num_cols as usize + 1)
        .x_label_formatter(&|v| format!("{}", (v / 10) ))
        .y_labels(num_rows as usize + 1)
        .y_label_formatter(&|v| format!("{}", (v / 10) ))
        .draw().unwrap();

    // Draw circles based on bitmatrix values
    // Draw circles based on bitmatrix values
    ctx.draw_series(
        (0..bitmatrix.dim().0).flat_map(|y| {
            (0..bitmatrix.dim().1).map(move |x| {
                let x_bit = bitmatrix[[y, x, 0]] as usize;
                let y_bit = bitmatrix[[y, x, 1]] as usize;
                let dot_type = x_bit + (y_bit << 1);
                let color = match dot_type {
                    0 => &BLACK, // UP
                    1 => &BLUE,  // LEFT
                    2 => &RED,   // RIGHT
                    3 => &GREEN, // DOWN
                    _ => &BLACK,
                };
                let mut x_x :i32 = x.clone() as i32;
                let mut y_y :i32 = (num_rows - 1) as i32 - y as i32;
                match dot_type {
                    0 => { // UP
                        x_x = x_x * 10;
                        y_y = y_y * 10 + 2;
                    }
                    1 => { // LEFT
                        x_x = (x_x * 10) - 2;
                        y_y = y_y * 10;
                    },
                    2 => { // RIGHT
                        x_x = x_x * 10 + 2;
                        y_y = y_y * 10;
                    }
                    3 => { // DOWN
                        x_x = x_x * 10;
                        y_y = (y_y * 10) - 2;
                    },
                    _ => {}
                };

                Circle::new((x_x as i32, y_y as i32), 5, color.filled())
            })
        })
    ).unwrap();

    Ok(())
}
