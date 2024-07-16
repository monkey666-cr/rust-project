fn hello_thread(n: u32) {
    println!("Hello from thread {n}");
}

fn do_match(i: u32) -> u32 {
    let mut n = i + 1;

    for _ in 0..10 {
        n *= 2;
    }

    n
}

fn main() {
    println!("Hello from main thread!");

    hello_thread(1);

    let mut thread_handles = vec![];

    for i in 0..10 {
        let thread_handle = std::thread::spawn(move || do_match(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|handle| {
        let res = handle.join().unwrap();

        println!("{}", res);
    });
}
