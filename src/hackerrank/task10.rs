pub fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts = [0; 101];

    for sock in ar {
        counts[*sock as usize] += 1;
    }

    let mut pairs = 0;

    for count in counts {
        pairs += count / 2;
    }

    pairs
}

#[test]
fn test0() {
    let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    let real = sock_merchant(&ar);
    let expected = 3;

    assert_eq!(real, expected);
}

#[test]
fn test1() {
    let ar = vec![1, 2, 1, 2, 1, 3, 2];

    let real = sock_merchant(&ar);
    let expected = 2;

    assert_eq!(real, expected);
}
