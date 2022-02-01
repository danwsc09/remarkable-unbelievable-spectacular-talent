fn main() {
    let res = length_of_last_word(" hello world\n".to_string());
    assert_eq!(res, "world".len() as i32);
}

pub fn length_of_last_word(s: String) -> i32 {
    let v: Vec<&str> = s.trim().split(' ').collect();
    v[v.len() - 1].len() as i32
}