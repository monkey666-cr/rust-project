fn do_match(i: u32) -> u32 {
    i * 2
}

fn main() {
    const N_THREAD: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();

    let mut thread_handles = vec![];

    let chunks = to_add.chunks(N_THREAD);

    for chunk in chunks {
        for i in chunk.to_owned() {
            let thread_handle = std::thread::spawn(move || do_match(i));
            thread_handles.push(thread_handle);
        }
    }

    let total = thread_handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .sum::<u32>();

    println!("{}", total);
}
