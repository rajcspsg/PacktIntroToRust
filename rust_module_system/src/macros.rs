#![macro_escape]
#[macro_export]

macro_rules! welcome {
    () => (
        println!("welcome to RUST macro!!!");
    )
}