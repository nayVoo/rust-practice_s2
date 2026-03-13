use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += arr[i][i];
        secondary_sum += arr[i][n - 1 - i];
    }

    (primary_sum - secondary_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap()
        .trim().parse::<usize>().unwrap();

    let mut arr = Vec::new();
    for _ in 0..n {
        let row: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        arr.push(row);
    }

    let result = diagonal_difference(&arr);
    writeln!(fptr, "{}", result).ok();
}
