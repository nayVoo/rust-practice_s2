use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 == v2 {
        if x1 == x2 {
            return "YES";
        } else {
            return "NO";
        }
    }

    let dx = x2 - x1;
    let dv = v1 - v2;

    if dv != 0 && dx % dv == 0 && dx / dv >= 0 {
        "YES"
    } else {
        "NO"
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let line = stdin_iterator.next().unwrap().unwrap();
    let parts: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = kangaroo(parts[0], parts[1], parts[2], parts[3]);
    writeln!(fptr, "{}", result).ok();
}
