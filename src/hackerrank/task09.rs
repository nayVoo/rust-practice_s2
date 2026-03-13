pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for bird in arr {
        counts[*bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..6 {
        if counts[i] > max_count {
            max_count = counts[i];
            result = i as i32;
        }
    }

    result
}

#[test]
fn test0() {
    let arr = vec![1, 4, 4, 4, 5, 3];

    let real = migratory_birds(&arr);
    let expected = 4;

    assert_eq!(real, expected);
}

#[test]
fn test1() {
    let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];

    let real = migratory_birds(&arr);
    let expected = 3;

    assert_eq!(real, expected);
}
