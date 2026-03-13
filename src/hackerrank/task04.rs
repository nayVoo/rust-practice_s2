pub fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades
        .into_iter()
        .map(|grade| {
            if grade < 38 {
                grade
            } else {
                let next_mult5 = ((grade / 5) + 1) * 5;
                if next_mult5 - grade < 3 {
                    next_mult5
                } else {
                    grade
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grading_students() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(input), expected);

        let input2 = vec![84, 29, 57];
        let expected2 = vec![85, 29, 57];
        assert_eq!(grading_students(input2), expected2);
    }
}
