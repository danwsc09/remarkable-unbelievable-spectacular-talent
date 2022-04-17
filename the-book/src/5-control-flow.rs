fn main() {
    /*
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3, or 4!");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    */
    /*
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
    */
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("liftoff!");

    println!(
        "100 deg F is {} deg C.",
        convert_temperature("c", 100).unwrap()
    );
    println!(
        "50 deg C is {} deg F.",
        convert_temperature("f", 50).unwrap()
    );
    println!(
        "50 deg C is {} deg T.",
        convert_temperature("t", 50).unwrap()
    );
}

fn convert_temperature(to: &str, degree: i32) -> Result<i32, std::io::Error> {
    match to {
        "c" => Ok((degree - 32) * 5 / 9),
        "f" => Ok(degree * 9 / 5 + 32),
        _ => panic!(),
    }
}
