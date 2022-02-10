mod b15469;
mod l58;
fn main() {
    b15469::solve();
    assert_eq!(
        l58::length_of_last_word(" hello world\n".to_string()),
        "world".len() as i32
    );
}
