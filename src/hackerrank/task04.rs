use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&g| {
        if g < 38 {
            g
        } else {
            let next_multiple_of_5 = ((g / 5) + 1) * 5;
            if next_multiple_of_5 - g < 3 {
                next_multiple_of_5
            } else {
                g
            }
        }
    }).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap()
        .trim().parse::<usize>().unwrap();

    let mut grades = Vec::new();
    for _ in 0..n {
        let grade = stdin_iterator.next().unwrap().unwrap()
            .trim().parse::<i32>().unwrap();
        grades.push(grade);
    }

    let result = grading_students(&grades);
    for g in result {
        writeln!(fptr, "{}", g).ok();
    }
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&g| {
        if g < 38 {
            g
        } else {
            let next_multiple_of_5 = ((g / 5) + 1) * 5;
            if next_multiple_of_5 - g < 3 {
                next_multiple_of_5
            } else {
                g
            }
        }
    }).collect()
}
