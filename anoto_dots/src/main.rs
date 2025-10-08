use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    //     [0,0]                  UP
    //       ^                    ^
    //       |                    |
    // [1,0]   [0,1]       LEFT <-+->  RIGHT
    //       |                    |
    //       v                    v
    //     [1,1]                 DOWN


    anoto_dots::gen_matrix()?;
    Ok(())
}
