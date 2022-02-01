mod l58;

fn main() {
    println!("Hello, world!");
    let res = l58::length_of_last_word(" hello world\n".to_string());
    assert_eq!(res, "world".len() as i32);
}
