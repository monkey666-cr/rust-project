fn greet(s: &str) {
    println!("hello {s}");
}

fn read_line() -> String {
    let mut input = String::new();

    // std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");

    input.trim().to_uppercase().to_string()
}

fn main() {
    let name = String::from("hello");

    greet(&name);

    println!("{name}");

    let input = read_line();

    print!("You typed: {input}");
}
