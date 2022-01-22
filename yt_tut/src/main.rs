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

    let mut x = 1;
    loop {
        if (x % 2 == 0) {
            println!("{} is even", x);
            x += 1;

            continue;
        }
        if (x > 10) {
            break;
        }
        x += 1;
    }

    let mut y = 1;
    while y <= 4 {
        println!("{}", y);
        y += 1;
    }

    for z in 1..10 {
        println!("FOR : {}", z);
    }

    let rand_string = "I am a random string";
    let (first, second) = rand_string.split_at(6);
    println!("Length of {}: {}", rand_string, rand_string.len());
    println!("{}/{}", first, second);

    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();
    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_char = chars.next();
    }

    let mut iter = rand_string.split_whitespace();
    let mut indiv_word = iter.next();

    loop {
        match indiv_word {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_word = iter.next();
    }

    let rand_string2 =
        "I am a random string\nThere are other strings like it\nThis string is the best";
    let mut lines = rand_string2.lines();
    let mut indiv_line = lines.next();

    loop {
        match indiv_line {
            Some(x) => println!("line:{}", x),
            None => break,
        }
        indiv_line = lines.next();
    }

    println!("Find Best : {}", rand_string2.contains("Best"));

    'outer: loop {
        let number: i32 = 10;
        println!("Pick a Number");

        loop {
            let mut line = String::new();
            let input = stdin().read_line(&mut line);

            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess {
                None => println!("Enter a Number"),
                Some(n) if n == number => {
                    println!("You Guessed It!");
                    break 'outer;
                }
                Some(n) if n < number => println!("Too Low"),
                Some(n) if n > number => println!("Too High"),
                Some(_) => println!("Error"),
            }
        }
    }
}
