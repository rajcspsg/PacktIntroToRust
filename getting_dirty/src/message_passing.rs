use std::sync::mpsc;
use std::sync::mpsc::Receiver;

type TokenType = i32;

struct Msg {
    typ: TokenType,
    val: String,
}

fn make_chann() -> Receiver<Msg> {
    let (tx,rx) = mpsc::sync_channel(1);
    let _ = tx.send(Msg {typ: 42, val: "Hello Commander, I'm your army man waiting for your command".to_string()});
    rx
}

fn main() {
    let rx = make_chann();
    // rx.recv.ok() {
    //     println!("The message received successfully!!!");
    // };
}