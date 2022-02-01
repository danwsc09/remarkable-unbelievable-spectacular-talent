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

    assert_eq!(10_i8 as i16, 10i16);
    // println!("pi: {}", std::f32);
    println!("2^4: {}", 2u16.pow(4));
    println!("-2: {}", (-2i16).abs());

    // vector
    let mut v = vec!["a man", "a plan", "a canal", "panama"];
    println!("v: {:?}", v);
    v.reverse();
    println!("v: {:?}", v);

    let mut v1: Vec<u8> = Vec::with_capacity(3);
    println!("vec: {:?}, cap: {}, len: {}", v1, v1.capacity(), v1.len());
    v1.push(1);v1.push(2);v1.push(3);v1.push(4);v1.push(5);
    println!("vec: {:?}, cap: {}, len: {}", v1, v1.capacity(), v1.len());
    
    let mut v2 = vec![1, 3, 5, 7, 9];
    println!("vec: {:?}, cap: {}, len: {}", v2, v2.capacity(), v2.len());
    v2.insert(3, 8);
    println!("vec: {:?}, cap: {}, len: {}", v2, v2.capacity(), v2.len());
    v2.remove(2);
    println!("vec: {:?}", v2);
    v2.pop();v2.pop();v2.pop();v2.pop();//v2.pop();
    let a = v2.pop().expect("array was empty");
    println!("a: {}", a);

    let mut laanguages: Vec<String> = std::env::args().skip(1).collect();
    for l in laanguages {
        println!("{} : {}", l,
        if l.len() % 2 == 0 {
            "functional"
        } else {
            "imperative"
        }
    )};
}

struct MyStruct {
    name: String,
    age: i32,
}

struct MyTupleStruct(String, i32);

enum Attend {
    OnTime,
    Late(u32),
}
