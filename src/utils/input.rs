use std::io;

pub fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}