pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let mut apples_count: i32 = 0;
    let mut oranges_count: i32 = 0;

    for d in apples {
        let position = a + d;
        if position >= s && position <= t {
            apples_count += 1;
        }
    }

    for d in oranges {
        let position = b + d;
        if position >= s && position <= t {
            oranges_count += 1;
        }
    }

    (apples_count, oranges_count)
}

#[test]
fn test0() {
    let s = 7;
    let t = 10;
    let a = 4;
    let b = 12;

    let apples = vec![2, 3, -4];
    let oranges = vec![3, -2, -4];

    let real = count_apples_and_oranges(s, t, a, b, &apples, &oranges);
    let expected = (1, 2);

    assert_eq!(real, expected);
}
