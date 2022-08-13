// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is

struct Matrix(f32, f32, f32, f32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Matrix {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    let trans_matrix = transpose(matrix);
    println!("Transpose:\n{}", trans_matrix);
}

fn transpose(m: Matrix) -> Matrix {
    let new_matrix = Matrix(m.0,m.2,m.1,m.3);
    new_matrix
}
