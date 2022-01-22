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
    /*
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
    */
    let rand_array = [1, 2, 3];
    println!("{}, {}", rand_array[0], rand_array[1]);
    println!("{}", rand_array.len());
    println!("Second 2: {:?}", &rand_array[1..3]);

    let mut vect1 = vec![1, 2, 3, 4, 5];
    println!("Item 2: {}", vect1[1]);
    for i in &vect1 {
        println!("Vect: {}", i);
    }

    vect1.push(6);
    vect1.push(7);
    vect1.pop();

    let rand_tuple = ("Ben", 55);
    let rand_tuple_2: (&str, i8) = ("Ben", 55);
    println!("Name : {}", rand_tuple_2.0);
    println!("Age : {}", rand_tuple_2.1);

    say_hello("Benjamin");
    println!("5 + 4 = {} ", get_sum(5, 4));

    let sum = get_sum;
    println!("6 + 4 = {}", sum(6, 4));

    let sum_nums = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7, 8));

    let num_ten = 10;
    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));

    // Ownership Error
    // let vect1 = vec![1, 2, 3];
    // let vect2 = vect1;
    // println!("vect1[0] = {}", vect1[0]);

    let prim_val = 1;
    let prim_val2 = prim_val;
    println!("prim_val: {}", prim_val);

    let vect2 = vec![1, 2, 3];
    println!("Sum of Vect : {}", sum_vects(&vect2));
    println!("Vect : {:?}", vect2);

    let mut circle1 = Circle {
        x: 10.0,
        y: 10.0,
        radius: 10.0,
    };

    println!("X: {}, Y: {}, R: {}", circle1.x, circle1.y, circle1.radius);
    println!("Circle Radius: {}", get_radius(&circle1));
    println!("Circle X: {}", circle1.get_x());
    println!("Circle Area: {}", circle1.area());

    let mut rect1 = Rectangle {
        height: 10.0,
        width: 10.0,
    };
    println!("Rect Area: {}", rect1.area());

    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
    let spiderman = Hero::Info {
        name: "Spiderman".to_owned(),
        secret: "Peter Parker".to_owned(),
    };

    get_info(hulk);
    get_info(quicksilver);
    get_info(spiderman);
}

enum Hero {
    Fast,
    Strong(i32),
    Info { name: String, secret: String },
}

fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast!"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info { name, secret } => {
            println!("{} is {}", name, secret);
        }
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

fn get_radius(circle: &Circle) -> f64 {
    circle.radius
}

impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

struct Rectangle {
    height: f64,
    width: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * 3.14159
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {
        sum += x;
        sum
    });
    return sum;
}

fn say_hello(name: &str) {
    println!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
    // return num1 + num2;
}
