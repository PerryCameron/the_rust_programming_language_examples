
// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// Convert temperatures between Fahrenheit and Celsius.

// Celsius to Fahrenheit Formula: (°C * 1.8) + 32 = °F
//
// Fahrenheit to Celsius Formula: (°F - 32) / 1.8 = °C


fn main() {
    println!("87.6 degrees fahrenheit is {:.1} Celsius", (87.6 - 32.0) / 1.8);
    println!("28.2 degrees celsius is {:.1} fahrenheit", (28.2 * 1.8) + 32.0);
}
