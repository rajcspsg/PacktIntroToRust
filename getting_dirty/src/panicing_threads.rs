use std::thread;

fn main() {
    let handle = thread::spawn(move || panic!("oops"));

    let result = handle.join();

    assert!(result.is_err());
}