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
