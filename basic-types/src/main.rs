fn main() {
    println!("this is a byte: {}", b'*');
    println!("isize min: {}", isize::MIN);
    println!("isize bits: {}", isize::BITS);
    let max_char = match char::MAX.to_digit(2) {
        Some(n) => n,
        None => 0,
    };
    println!("max char: {}", max_char);
    // Box::<Attend> myBox = Box::new(Late(15));
    
    let c = 5u32;
    let d = c;
    println!("c: {}", c);
    println!("*d: {}", d);
    println!("c: {}", c);

    let big_val = std::i32::MAX;
    let x = big_val.wrapping_add(1);
    println!("MAX: {}", big_val);
    println!("big_val + 1: {}", x);

    println!("byte literal b'X': {}", b'X');
    println!("byte literal b\\x42: {}", b'\x42');
}

struct MyStruct {
    name: String,
    age: i32,
}

struct MyTupleStruct (String, i32);

enum Attend {
    OnTime,
    Late(u32)
}

