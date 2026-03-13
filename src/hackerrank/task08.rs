pub fn breaking_records(scores: &[i32]) -> (i32, i32) {
    let mut max_score = scores[0];
    let mut min_score = scores[0];

    let mut max_breaks: i32 = 0;
    let mut min_breaks: i32 = 0;

    for score in scores {
        if *score > max_score {
            max_score = *score;
            max_breaks += 1;
        }

        if *score < min_score {
            min_score = *score;
            min_breaks += 1;
        }
    }

    (max_breaks, min_breaks)
}

#[test]
fn test0() {
    let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];

    let real = breaking_records(&scores);
    let expected = (2, 4);

    assert_eq!(real, expected);
}

#[test]
fn test1() {
    let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];

    let real = breaking_records(&scores);
    let expected = (4, 0);

    assert_eq!(real, expe }
