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
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);
    println!("vec: {:?}, cap: {}, len: {}", v1, v1.capacity(), v1.len());

    let mut v2 = vec![1, 3, 5, 7, 9];
    println!("vec: {:?}, cap: {}, len: {}", v2, v2.capacity(), v2.len());
    v2.insert(3, 8);
    println!("vec: {:?}, cap: {}, len: {}", v2, v2.capacity(), v2.len());
    v2.remove(2);
    println!("vec: {:?}", v2);
    v2.pop();
    v2.pop();
    v2.pop();
    v2.pop(); //v2.pop();
    let a = v2.pop().expect("array was empty");
    println!("a: {}", a);

    let mut languages: Vec<String> = std::env::args().skip(1).collect();
    println!("languages: {:?}", languages);
    for l in languages {
        println!(
            "{} : {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        )
    }

    println!("===================");
    const MAX_NUMS: usize = 500;
    let mut sieve = [true; MAX_NUMS];
    for i in 2..23 {
        if sieve[i] {
            let mut j = i + i;
            while j < MAX_NUMS {
                sieve[j] = false;
                j += i;
            }
        }
    }
    for i in 2..MAX_NUMS {
        if sieve[i] {
            print!("{} ", i);
        }
    }
    println!("");
    println!("=========VEC==========");
    let mut myV = vec![2, 3, 5, 7];
    assert_eq!(myV.iter().fold(1, |a, b| a * b), 210);
    assert_eq!(myV.iter().fold(0, |a, b| a + b), 17);
    myV.push(11);
    myV.push(13);
    assert_eq!(myV.iter().fold(0, |a, b| a + b), 41);

    let mut myV1 = Vec::new();
    myV1.push("hello");
    myV1.push("how");
    myV1.push("are");
    myV1.push("you");
    assert_eq!(myV1, vec!["hello", "how", "are", "you"]);

    let mut myV2: Vec<i32> = (0..5).collect();
    assert_eq!(myV2, vec![0, 1, 2, 3, 4]);
    println!("");
    println!("=========SLICES==========");
    let v1: Vec<f64> = vec![0.0, 0.707, -1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, -1.0, 0.707];

    let sv = &v1;
    let sa = &a;
    print(sv);
    print(sa);

    println!("");
    println!("=========STRINGS==========");
    println!("-----String Literal-----");
    let speech = "\"OUCH!\" said the well.\n";
    let another_str = "hello,
    said the man.";
    let yet_another_str = "hello, \
    said the man.";
    println!("{}", speech);
    println!("{}", another_str);
    println!("{}", yet_another_str);
    
    let raw_string1 = r"Windows path: C:\Program Files\Gorillas";
    let raw_string2 = r###"
    This raw string started with 'r###"'. 
    Therefore, it doesn't end until we reach a quote mark ('"')
    followed immediately by three pound signs('###')."###;
    println!("{}", raw_string1);
    println!("{}", raw_string2);
    
    println!("");
    println!("-----Byte Strings-----");
    let method = b"GET";
    println!("method: {:?}", method);
    
    println!("-----More Strings-----");
    let mut noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "😃_😃";
    println!("noodles:{}, oodles:{}, poodles:{}", noodles, oodles, poodles);
    
    println!("noodles.len(): {}", noodles.len());
    println!("noodles.chars().count(): {}", noodles.chars().count());
    println!("oodles.len(): {}", oodles.len());
    println!("oodles.chars().count(): {}", oodles.chars().count());
    println!("poodles.len(): {}", poodles.len());
    println!("poodles.chars().count(): {}", poodles.chars().count());

    noodles.pop();
    println!("new noodles: {}", noodles);
    println!("-----String------");
}

fn print(arr: &[f64]) {
    for elt in arr {
        println!("{:?}", elt);
    }
    println!("==");
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
