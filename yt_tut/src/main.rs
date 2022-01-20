// use std::{f32, f64, i16, i32, i64, i8, isize, u16, u32, u64, u8, usize};

use std::io::stdin;

fn main() {
    println!("Hello, world!");

    let num = 10;
    let mut age: i32 = 40;
    println!("Max i8 {}", i8::MAX);
    println!("Min i8 {}", i8::MIN);
    println!("Max i64 {}", i64::MAX);
    println!("Min i64 {}", i64::MIN);
    println!("Max isize {}", isize::MAX);
    println!("Min isize {}", isize::MIN);

    let is_it_true: bool = true;
    let let_x: char = 'x';
    println!("I am {} years old", age);

    let (f_name, l_name) = ("Steve", "Jobless");
    println!("fname: {}, lname: {}", f_name, l_name);

    println!("It is {0} that {1} is {0}", is_it_true, let_x);
    println!("{:.2}", 1.235);
    println!("B: {:b}, H: {:x} 0: {:o}", 10, 10, 10);
    println!("{ten:>ws$}", ten = 10, ws = 5);
    println!("{ten:0ws$}", ten = 10, ws = 5);

    println!("5 + 4 = {}", 5 + 4);
    println!("5 / 3 = {}", 5 / 3);
    println!("5 % 3 = {}", 5 % 3);

    let mut neg_4 = -4i32;
    println!("abs(-4) = {}", neg_4.abs());
    let mut pos_5: i32 = 5;
    println!("5 ^ 3 = {}", pos_5.pow(3)); // .sqrt, .cbrt, .round, .floor, .ceil, .exp, .ln, .log10, to_radians, to_degrees, max, min, sin
    println!("3.14 to deg = {}", 3.14f64.to_degrees());

    // conditionals
    let age_old = 20;
    if age_old == 5 {
        println!("Go to Kindergarten");
    } else if (age_old > 5) && (age_old <= 18) {
        println!("Go to grade {}", age_old - 5);
    } else if (age_old <= 25) && (age_old > 18) {
        println!("Go to college");
    } else {
        println!("Do what you want.");
    }

    println!("!true = {}", !true);
    println!("true || false = {}", true || false);

    let can_vote = if (age_old >= 18) { true } else { false };
    println!("can vote: {}", can_vote);
}
