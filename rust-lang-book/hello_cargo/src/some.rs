#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_ne!(gcd(2 * 3 * 5 * 11 * 17,
                    3 * 7 * 11 * 13 * 19),
                    3 * 11);
}