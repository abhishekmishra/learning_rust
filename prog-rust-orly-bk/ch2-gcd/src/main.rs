fn main() {
    println!("Gcd of 12 and 15 is {}", gcd(12, 15));

    // this will panic
    // because we are passing 0
    // println!("Gcd of 12 and 0 is {}", gcd(12, 0));
}

fn gcd(mut n:u64, mut m:u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11, 2 * 3 * 7), 2 * 3);
}