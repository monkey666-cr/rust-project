fn main() {
    let my_vec = vec![1, 2, 3, 4, 5];

    if let Some(value) = my_vec.get(2) {
        println!("value: {value}")
    }

    unsafe {
        let value = my_vec.get_unchecked(3);
        println!("{value}");
    }
}
