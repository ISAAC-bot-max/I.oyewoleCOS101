fn main(){

    //using Vec::new()
    let p: Vec<i64> = Vec::new();

    //printing the size of vector 
    println!("\nThe length of Vec::new is {}",p.len());

    //using macro 
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing the size of the vector
    println!("\nThe length of vec macro is : {}",v.len()); 
}