use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn breaking_records(scores: &[i32]) -> [i32; 2] {
    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_count = 0;
    let mut min_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    [max_count, min_count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap()
        .trim().parse::<usize>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    writeln!(&mut fptr, "{} {}", result[0], result[1]).ok();
}
