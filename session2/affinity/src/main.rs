fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    println!("{:#?}", core_ids);

    let handles = core_ids
        .into_iter()
        .map(|id| {
            std::thread::spawn(move || {
                let success = core_affinity::set_for_current(id);
                if success {
                    println!("Hello from a thread on core {id:?}");
                } else {
                    println!("Unable to set affinity to code {id:?}");
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
