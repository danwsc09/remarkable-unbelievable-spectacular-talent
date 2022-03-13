use std::{collections::HashMap, i32};

type Table = HashMap<String, Vec<String>>;

fn main() {
    println!("Hello, world!");
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    table.insert(
        "Me!".to_string(),
        vec!["Me".to_string(), "Myself".to_string(), "And I".to_string()],
    );
    show(&table);
    sort_works(&mut table);
    show(&table);

    println!("=======Pass by Value vs Reference=======");
    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
    assert!(y == 64);

    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    };
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
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

    struct APoint {
        x: i32,
        y: i32,
    };
    let point = APoint { x: 10, y: 20 };
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
    // assert_eq!(*r, 1);

    static mut STASH: &i32 = &128;
    static WORTH_POINTING_AT: i32 = 100;
    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }
    f(&WORTH_POINTING_AT);

    fn g<'a>(p: &'a i32) {}
    let x = 10;
    g(&x);
    // f(&x);

    {
        let s;
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        println!("s: {}", s);
    }

    struct S<'a> {
        r: &'a i32,
    }
    let s;
    {
        let x = 10;
        s = S { r: &x };
        println!("s.r: {}", s.r);
    }
    // println!("s.r: {}", s.r);

    // struct T {
    //     s: S<'static>,
    // }
    // static x: i32 = 10;
    // let t: T = T {s: S {r: &x}};
    struct T<'a> {
        s: S<'a>,
    }
    let x: i32 = 10;
    let t: T = T { s: S { r: &x } };

    struct Ss<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = Ss { x: &x, y: &y };
            r = s.x;
            println!("r: {}", r);
        }
    }

    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    x += 10;
    let m = &mut x;
    println!("x is {}", x);
    // println!("r1 is {}", r1);

    let mut y = 20;
    let m1 = &mut y;
    // let m2 = &mut y;
    // let z = y;
    *m1 += 10;
}

fn factorial(n: usize) -> usize {
    (1..n + 1).fold(1, |a, b| a * b)
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {}:", artist);
        for work in works {
            println!("  - {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort_unstable()
    }
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    if v.len() == 0 {
        return &0;
    }
    let mut min = &v[0];
    for num in &v[1..] {
        if *num < *min {
            min = num;
        }
    }
    min
}
