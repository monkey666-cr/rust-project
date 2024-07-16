use std::sync::mpsc;

enum Command {
    SayHello,
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Say Helllo"),
                Command::Quit => {
                    println!("Quitting");
                    break;
                }
            }
        }
    });

    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap();
    }

    tx.send(Command::Quit).unwrap();

    handle.join().unwrap();
}
