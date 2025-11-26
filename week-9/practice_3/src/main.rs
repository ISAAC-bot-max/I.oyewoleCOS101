use std::fs;
use std::fs::File;

fn main(){
    let  file = File::create("data.txt").unwrap();
    fs::remove_file("data.txt").expect("could not remove file");
    println!("File is removed");
}