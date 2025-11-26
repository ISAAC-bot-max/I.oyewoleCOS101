use std::fs::OpenOptions; 
use std::io::Write;
use std::fs::File;
use std::io;


fn main(){

    let mut file = File::create("data.txt").unwrap();
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Cannot open file");
    file.write_all("\nHello Class".as_bytes()).unwrap();
    file.write_all("\nThis is the appendage to the document".as_bytes()).unwrap();
    file.write_all("\nMr Chukwudi is my Computer Science\n".as_bytes());

    println!("Enter anything:__");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input1 = input.trim();
    file.write_all(input1.as_bytes());
    println!("File append success");
}