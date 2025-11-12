fn main(){
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is :{}",num);
}

fn mutate_num_to_zero(paranum_num:&mut i32){
    *paranum_num = *paranum_num*0;  //de reference
    println!("paranum_num value is :{}",paranum_num);
}