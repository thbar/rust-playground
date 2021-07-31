// add trait in scope, so that e.g. u64 can use it
use std::str::FromStr;

// TODO: add unit tests

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // TODO: add assertions
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        // TODO: list traits implemented by u64, or all types implementing from_str, after importing FromStr above
        numbers.push(u64::from_str(&arg).expect("a number is expected"));
    }
    println!("Numbers: {:?}", numbers);
    let mut n = numbers[0];
    for m in &numbers[1..] {
        n = gcd(n, *m);
    }
    println!("Result is {}", n);
}
