//  Exercise 1 - 8.3
// https://doc.rust-lang.org/book/ch08-03-hash-maps.html found at bottom of page
//  Given a list of integers, use a vector and return the median (when sorted, the value
//  in the middle position) and mode (the value that occurs most often; a hash map will
//  be helpful here) of the list.
use std::collections::HashMap;

fn main() {
    let mut a = vec![ 12, 14, 15, 3, 12, 9, 5, 27, 18, 19, 20, 30, 6, 8];

    find_median(&mut a);
    // find_mode(&a);

    println!("Sorted array= {:?}", a);
    println!("a length = {}", a.len());
    println!("mode {}", mode(&a));
}

fn mode(a: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for v in a.iter() {
        let count = occurrences.entry(v).or_insert(0);
        *count += 1;
    }

    let mut maximum = occurrences.get(&a[0]).expect("No value in occurrences for first value of int_list");
    let mut result = &a[0];

    for (key, value) in &occurrences {
        if value > maximum {
            result = *key;
            maximum = value;
        }
    }
    *result
}

fn find_median(a: &mut Vec<i32>) {
    a.sort();
    let b = (a.len() as f64 / 2 as f64).floor();
    let c: usize = b as usize;
    let mut result = 0;
    if let 0 = c % 2 {
        result = a[c];
    } else {
        result = ((a[(c - 1)] as f64 + a[c] as f64) / 2.0) as i32);
    }
}
