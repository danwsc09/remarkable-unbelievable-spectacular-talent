fn main() {
    let a = 24;
    let b = 36;
    let result = gcd(a, b);
    println!("Result: {}", result);
    // let this_will_error = gcd(0, 5);
    // println!("Error: {}", this_will_error);
    let x: f32 = 1.57;
    let y = {
        println!("x is {}", x);
        x.cos()
    };
    println!("y is {}", y);
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}
