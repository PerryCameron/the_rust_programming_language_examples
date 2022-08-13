
// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// Generate the nth Fibonacci number.


fn main() {
    let numb: i64 = 9;
    println!("the nth Fibanacci numnber is {} ", fib(numb));
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    return fib(n -1) + fib(n -2)
}