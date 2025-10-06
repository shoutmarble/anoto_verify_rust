use anoto_verify_rust::*;

fn main() {
    // Use the default Anoto 6x6 A4 fixed codec
    let codec = anoto_6x6_a4_fixed();

    // Generate a bit-matrix for section (10, 2) with shape (60, 60)
    let bits = codec.encode_bitmatrix((60, 60), (10, 2));
    println!("Generated bit-matrix with shape: {:?}", bits.dim());

    // Decode position from a 6x6 sub-matrix at position (5, 10)
    let sub = bits.slice(ndarray::s![10..16, 5..11, ..]).to_owned();
    let pos = codec.decode_position(&sub).unwrap();
    println!("Decoded position: {:?}", pos); // Should be (5, 10)

    // Decode section from the same sub-matrix
    let sec = codec.decode_section(&sub, pos).unwrap();
    println!("Decoded section: {:?}", sec); // Should be (10, 2)

    // Decode rotation (requires 8x8 matrix)
    let sub8 = bits.slice(ndarray::s![0..8, 0..8, ..]).to_owned();
    let rot = codec.decode_rotation(&sub8).unwrap();
    println!("Pattern rotation: {} degrees", rot * 90);
}
