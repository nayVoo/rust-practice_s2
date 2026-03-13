pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count: i32 = 0;

    let mut candidate = 1;
    while candidate <= 100 {
        let mut valid = true;

        for x in a {
            if candidate % x != 0 {
                valid = false;
                break;
            }
        }

        if valid {
            for y in b {
                if y % candidate != 0 {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            count += 1;
        }

        candidate += 1;
    }

    count
}

#[test]
fn test0() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];

    let real = get_total_x(&a, &b);
    let expected = 3;

    assert_eq!(real, expected);
}
