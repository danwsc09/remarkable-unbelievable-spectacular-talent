use std::io;
use std::str::FromStr;

pub fn solve() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    let vec: Vec<&str> = buffer.trim().split(" ").collect();
    let n = <usize>::from_str(&vec[0]).expect("error while parsing");
    let m = <usize>::from_str(&vec[1]).expect("error while parsing");

    print_all_sequences(n, m);
}

pub fn print_all_sequences(n: usize, m: usize) {
    let mut vec: Vec<usize> = Vec::new();
    helper(n, m, &mut vec);
}

pub fn helper(n: usize, m: usize, vec: &mut Vec<usize>) {
    if vec.len() == m {
        for num in vec {
            print!("{} ", num)
        }
        print!("\n");
        return;
    }
    for i in 1..(n + 1) {
        if !vec.contains(&i) {
            vec.push(i);
            helper(n, m, vec);
            vec.pop();
        }
    }
}
