use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

// Compute GCD
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Compute LCM
fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    // LCM of all elements in a
    let mut lcm_a = a[0];
    for &num in a.iter().skip(1) {
        lcm_a = lcm(lcm_a, num);
    }

    // GCD of all elements in b
    let mut gcd_b = b[0];
    for &num in b.iter().skip(1) {
        gcd_b = gcd(gcd_b, num);
    }

    // Count multiples of lcm_a that divide gcd_b
    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_line = stdin_iterator.next().unwrap().unwrap();
    let _ = first_line.trim(); // n and m, not needed

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = get_total_x(&a, &b);
    writeln!(fptr, "{}", result).ok();
}
