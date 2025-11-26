use std::fs::OpenOptions; 
use std::io::Write;
use std::fs::File;


fn main(){

    let mut file = File::create("data.txt").unwrap();
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Cannot open file");
    file.write_all("\nHello Class".as_bytes()).unwrap();
    file.write_all("\nThis is the appendage to the document".as_bytes()).unwrap();
    println!("File append success");
}