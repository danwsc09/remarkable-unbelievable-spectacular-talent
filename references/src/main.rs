use std::collections::HashMap;

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
    // assert_eq!(*r, 1);
}

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
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