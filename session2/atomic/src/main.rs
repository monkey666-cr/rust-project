use std::sync::atomic::AtomicI32;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = vec![];

    for _ in 0..100 {
        let handle = std::thread::spawn(|| {
            COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
