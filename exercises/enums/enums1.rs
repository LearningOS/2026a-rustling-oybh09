// enums1.rs
//

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String),
}

fn main() {
    
    println!("{:?}", Message::Quit(String::from("stop!")));
    println!("{:?}", Message::Echo(String::from("echo echo")));
    println!("{:?}", Message::Move(String::from("move")));
    println!("{:?}", Message::ChangeColor(String::from("change")));
}
