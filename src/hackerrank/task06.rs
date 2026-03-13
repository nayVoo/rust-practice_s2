pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return String::from("NO");
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[test]
fn test0() {
    let real = kangaroo(0, 3, 4, 2);
    let expected = String::from("YES");
    assert_eq!(real, expected);
}

#[test]
fn test1() {
    let real = kangaroo(0, 2, 5, 3);
    let expected = String::from("NO");
    assert_eq!(real, expected);
}
