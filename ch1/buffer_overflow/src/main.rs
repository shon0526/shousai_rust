fn main() {
    let fruit = vec!["banana", "apple", "orange"];
    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, "banana");
}
