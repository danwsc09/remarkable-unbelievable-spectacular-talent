use std::rc::Rc;

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
    println!(
        "noodles:{}, oodles:{}, poodles:{}",
        noodles, oodles, poodles
    );

    println!("noodles.len(): {}", noodles.len());
    println!("noodles.chars().count(): {}", noodles.chars().count());
    println!("oodles.len(): {}", oodles.len());
    println!("oodles.chars().count(): {}", oodles.chars().count());
    println!("poodles.len(): {}", poodles.len());
    println!("poodles.chars().count(): {}", poodles.chars().count());

    noodles.pop();
    println!("new noodles: {}", noodles);

    println!("-----String------");
    let my_str1 = "pewpew".to_string();
    let my_slice1 = &my_str1[1..3];
    println!("my str: {}, my slice: {}", my_str1, my_slice1);
    let formated_str1 = format!("{}N{:02}E", 24, 5);
    assert_eq!(formated_str1, "24N05E");

    let str_vec = vec!["veni", "vidi", "vici"];
    assert_eq!(str_vec.concat(), "venividivici");
    assert_eq!(str_vec.join("_ "), "veni_ vidi_ vici");

    let mut str_array = [""; 5];
    str_array[0] = "hello";

    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("a_a".replace("a", "b"), "b_b");
    assert_eq!("  clean \n ".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

    println!("==============================");
    println!("==============================");
    println!("========OWNERSHIP=======");
    let my_nums = vec![1, 2, 3, 4];
    // let sum = get_sum_move(my_nums);
    // println!("can i access my_nums? {:?}", my_nums); // no

    let sum = get_sum_borrow(&my_nums);
    println!("sum: {}", sum);
    println!("can i access my_nums? {:?}", my_nums); // yes

    let box_result = make_box();
    let replaced_result = box_result.replace("s", "z");
    println!("replaced: {}", replaced_result);

    let mut people = Vec::new();
    people.push(Person {
        birth: 1525,
        name: "Palentina".to_string(),
    });
    people.push(Person {
        birth: 1563,
        name: "Dowland".to_string(),
    });
    people.push(Person {
        birth: 1632,
        name: "Lully".to_string(),
    });

    for person in &people {
        println!("Name: {}, Birth Year: {}", person.name, person.birth);
    }

    println!("================================");
    println!("===========MOVES==========");
    let my_vector = vec!["hi", "hello", "bye"];
    let my_vector1 = my_vector;
    // let my_vector2 = my_vector; // error

    let x = vec![10, 20, 30];
    let res1 = rand::random::<bool>();
    if res1 {
        empty_fun1(x);
    } else {
        empty_fun2(x);
    }

    let mut my_v1 = Vec::new();
    for i in 101..106 {
        my_v1.push(i.to_string());
    }
    println!("my v1: {:?}", my_v1);
    let third = &my_v1[2];
    // let fifth = my_v1[4];
    println!("third: {}", third);

    // 1. pop off from end
    let fifth = my_v1.pop().unwrap();
    println!("fifth: {}", fifth);

    // 2. Move a value out of the middle of vector, and move the last into its spot
    let second = my_v1.swap_remove(1);
    println!("second: {}, vector: {:?}", second, my_v1);

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut my_v1[2], "substitute".to_string());
    println!("third: {}, vector: {:?}", third, my_v1);

    struct SomePerson {
        name: Option<String>,
        birth: i32,
    }

    let mut people1 = Vec::new();
    people1.push(SomePerson {
        birth: 1993,
        name: Some("Dan".to_string()),
    });

    // let the_name = people1[0].name.take();
    let the_name = std::mem::replace(&mut people1[0].name, None);
    // println!("the name: {}", the_name);
    assert_eq!(the_name, Some("Dan".to_string()));
    for person in &people1 {
        println!("birth: {}, name: {:?}", person.birth, person.name);
    }

    let mut people2 = Vec::new();
    people2.push(Person {
        birth: 1993,
        name: "Dan".to_string(),
    });
    let first_name = &mut people2[0].name;
    first_name.push('d');

    println!("============COPY TYPES============");
    let l = Label { number: 3 };
    print_label(l);
    println!("My label number is : {}", l.number);

    println!("");
    println!("========Reference Count========");
    let rc_str: Rc<String> = Rc::new("abcdef".to_string());
    let rc_str1 = rc_str.clone();
    let rc_str2 = rc_str.clone();
    let mut some_str: String = "abcdef".to_string();

    println!("{} contains cd: {}", rc_str, rc_str.contains("cd"));
    assert_eq!(rc_str1.find("de"), Some(3));
    println!("{} is cloned.", rc_str2);
    // rc_str.push_str("ghi");
    some_str.push_str("ghi");

    println!("=======Pass by Value vs Reference=======");
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
    assert!(y == 64);

    struct Anime { name: &'static str, bechdel_pass: bool };
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut a_vec = vec![2002, 1993];
    println!("{:?}", a_vec);
    a_vec.sort();
    // (&mut a_vec).sort();
    println!("{:?}", a_vec);

    let x = 10;
    let y = 20;
    let mut r = &x;
    r = &y;
    println!("r: {} should be 20", *r);

    struct APoint {x: i32, y: i32};
    let point = APoint {x: 10, y: 20};
    let r = &point;
    let rr = &r;
    let rrr = &rr;
    println!("x: {}", rrr.x);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx == rry);
    assert!(rrx <= rry);

    assert!(!std::ptr::eq(rrx, rry));

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    let r;
    {
        let x = 1;
        r = &x;
    }
    assert_eq!(*r, 1);
}

/*  =============================================
================End of main()================
=============================================
*/

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

#[derive(Clone, Copy)]
struct Label {
    number: i32,
}
fn print_label(l: Label) {
    println!("STAMP: {}", l.number);
}

fn empty_fun1(arr: Vec<i32>) {}
fn empty_fun2(arr: Vec<i32>) {}

fn make_box() -> Box<String> {
    let mut my_str = "some string".to_string();
    Box::new(my_str)
}

fn get_sum_borrow(arr: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in arr {
        total += num;
    }
    total
}

fn get_sum_move(arr: Vec<i32>) -> i32 {
    let mut total = 0;
    for num in arr {
        total += num;
    }
    total
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

struct Person {
    name: String,
    birth: i32,
}
