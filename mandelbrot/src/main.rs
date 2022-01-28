extern crate num;

use num::Complex;
use std::str::FromStr;

fn main() {
    let result: (i32, i32) = match parse_pair("400x600", 'x') {
        Some(tuple) => tuple,
        None => (0, 0),
    };
    println!("Parse pair: {:?}", result);

    let mut vec = vec![1, 2, 3];
    let int_slice = &mut vec[..];
    int_slice[1] = 10;
    // println!("vec: {:?}", vec);
    println!("int_slice: {:?}", int_slice);

    println!("num: {:?}", parse_complex("3.0,-0.625"));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex { re, im }),
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("3,2"), Some(Complex { re: 3.0, im: 2.0 }));
    assert_eq!(parse_complex("3,"), None);
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right> where <sep> is the character
/// given by the `separator` argument.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse correctly,
/// return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<i32>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Try to determine if 'c' is in the Mandelbrot set, using at most 'limit' iterations to decide.
///
/// If 'c' is not a member, return 'Some(i)', where `i` is the number of iterations it took for `c`
/// to leave the circle of radius two centered on the origin.
/// If `c` seems to be a member, return `None`.
#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}
