pub fn length_of_last_word(s: String) -> i32 {
    let v: Vec<&str> = s.trim().split(' ').collect();
    v[v.len() - 1].len() as i32
}
