# Elliptic Curve Point Compression and Decompression in Rust

This Rust project implements elliptic curve point compression and decompression over a finite field. It reduces storage requirements for elliptic curve points by encoding only the $x$-coordinate and a single bit representing the $y$-coordinate's parity. The decompression process reconstructs the full point from the compressed data.

## Features

- **Elliptic Curve Arithmetic**: Operations over finite fields.
- **Point Compression**: Encodes elliptic curve points as $x$-coordinate and $y$-parity.
- **Point Decompression**: Reconstructs points from compressed data.
- **Finite Field Arithmetic**: Modular arithmetic for field operations.

## Prerequisites

Ensure [Rust](https://www.rust-lang.org/tools/install) is installed.

## Installation

1. Clone the repository:
```bash
   git clone https://github.com/yourusername/Elliptic_Curve_Point_Compression_and_Decompression_in_Rust.git
   cd Elliptic_Curve_Point_Compression_and_Decompression_in_Rust
```
2. Build the project:
```
cargo build
```
3. Run the project:
```
cargo run
```
## Usage
Upon running the project, the output demonstrates the compression and decompression of an elliptic curve point. Example:
```
Compressed: Some((2, true))
Decompressed: Some(ECPoint {x: 2, y: 5, is_infinity: false })
```
## Explanation
- **Compression**: Encodes a point on the curve using its $x$-coordinate and the parity of $y$.
- **Decompression**: Solves the curve equation $y^2 \equiv x^3 + ax + b$ mod $p$ to reconstruct $y$, using the parity bit to determine the correct $y$.

## Code

### EllipticCurve Struct

Defines the elliptic curve equation:

$y^2 \equiv x^3 + ax + b$ mod $p$


#### Methods

- `is_on_curve`: Verifies whether a point satisfies the curve equation.
- `compress`: Compresses a point into its $x$-coordinate and $y$-parity.
- `decompress`: Reconstructs a point from compressed data.

### ECPoint Struct

Represents a point on the elliptic curve with fields:

- $( x, y )$: Coordinates.
- `is_infinity`: Boolean flag for the point at infinity.

## Example Curve Parameters

- **Curve Equation**: $y^2 = x^3 + 3x + 4$ mod $7$
- **Parameters**:
  - $a = 3 , b = 4 , p = 7$
- **Example Point**:
  - $x = 2 , y = 5$
## Dependencies
Add the following to your `Cargo.toml`:
```
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"
```
## Contributions
Contributions are welcome! Fork the repository, create a new branch for your feature, and submit a pull request.
