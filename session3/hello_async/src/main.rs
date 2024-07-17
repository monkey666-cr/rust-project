use std::time::Duration;

use futures::{executor::block_on, future::join_all, join};

async fn say_hello() {
    println!("Hello");

    join!(second_function(), say_goodbye());

    let n = double(4).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(3)];
    let results = join_all(futures).await;
    println!("{results:?}");
}

async fn second_function() {
    println!("Hello again");
}

async fn say_goodbye() {
    println!("Goodbye");
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    // block_on(say_hello());
    std::thread::spawn(|| block_on(say_hello()));

    std::thread::sleep(Duration::from_secs(3));
}
