// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut x = 100;
    println!("{}",x); // x = 100
    let y = &mut x;
    // if we want to make an assertion about the value in y, we have to use
    // *y to follow the reference to the value it’s pointing to (hence dereference)
    // so the compiler can compare the actual value. Once we dereference y, we have
    // access to the integer value y is pointing to
    *y += 100;
    println!("{}",x); // x = 200
    let z = &mut x;
    *z += 1000; // dereference which points back to x
    println!("{}",x); // x = 1200
    assert_eq!(x, 1200);
}
