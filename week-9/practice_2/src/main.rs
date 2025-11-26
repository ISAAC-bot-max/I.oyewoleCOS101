use std::io::Read;
use std::fs::File;

fn main(){
    let mut file = File::create("Welcome_message.txt").unwrap();
    let mut file = File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}