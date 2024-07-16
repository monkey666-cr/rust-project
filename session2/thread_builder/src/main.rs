use std::thread;

fn my_thread() {
    println!(
        "Hello from a thread named {}",
        thread::current().name().unwrap()
    );
}

fn main() {
    my_thread();

    let builder = thread::Builder::new().name("Named Thread".to_string());

    let handle = builder.spawn(my_thread).unwrap();

    handle.join().unwrap();
}
