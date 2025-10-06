# anoto_verify_rust

A Rust library for encoding and decoding 2D locations based on the [Anoto](https://www.anoto.com/) dot pattern approach. This is a Rust port of the Python [py-microdots](https://github.com/cheind/py-microdots) library.

## Features

- **Encoding**: Generate bit-matrices for specific sections
- **Decoding**: Extract position coordinates from bit-matrices
- **Section Decoding**: Determine section coordinates
- **Rotation Detection**: Identify pattern orientation in 90° steps

## Usage

```rust
use anoto_verify_rust::*;

// Use the default Anoto 6x6 A4 fixed codec
let codec = anoto_6x6_a4_fixed();

// Generate a bit-matrix for section (10, 2)
let bits = codec.encode_bitmatrix((60, 60), (10, 2));

// Decode position from a 6x6 sub-matrix
let sub = bits.slice(ndarray::s![0..6, 0..6, ..]).to_owned();
let pos = codec.decode_position(&sub).unwrap();
println!("Position: {:?}", pos);

// Decode section
let sec = codec.decode_section(&sub, pos).unwrap();
println!("Section: {:?}", sec);

// Decode rotation (requires 8x8 matrix)
let sub8 = bits.slice(ndarray::s![0..8, 0..8, ..]).to_owned();
let rot = codec.decode_rotation(&sub8).unwrap();
println!("Rotation: {}", rot);
```

## Testing

The library includes comprehensive tests that verify the output matches the Python `py-microdots` implementation:

```bash
cargo test
```

## References

This implementation is based on the Anoto coding system and the [py-microdots](https://github.com/cheind/py-microdots) library by Christoph Heindl.

For more information about the Anoto codec, see:
- [py-microdots Paper](https://zenodo.org/record/7361722)
- Christoph Heindl, "py-microdots: Position Encoding in the Euclidean Plane Based on the Anoto Codec", 2023

## License

This project follows the same license terms as the original py-microdots library.
