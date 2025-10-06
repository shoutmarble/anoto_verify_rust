use anoto_verify_rust::*;

fn main() {
    let anoto = anoto_6x6_a4_fixed();
    
    // Check the MNS sequence
    println!("MNS first 8 values:");
    let m = anoto.encode_bitmatrix((10, 10), (0, 0));
    
    // Print what we're actually getting
    println!("Matrix dimensions: {:?}", m.dim());
    for y in 0..8 {
        for x in 0..8 {
            print!("[{},{}]: ({},{})  ", y, x, m[[y,x,0]], m[[y,x,1]]);
        }
        println!();
    }
}
