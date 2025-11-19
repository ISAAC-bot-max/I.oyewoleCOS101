//Method to print the get value 
fn value(n:Option<&char>){

    println!("Element of vector {:?}",n);

}
fn main(){

    let rust = vec!['R','U','S','T','A','C','I','A','N'];
    let mut input1 = String::new();
    println!("\nEnter an index value between (0-8)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");


    //index is the no negative value which is smaller than the size of the vector 
    let index:usize = input1.trim().parse().expect("Invalid input");


    //getting value at given index value 
    let ch: Option<&char> = rust.get(index);
    value(ch);

}