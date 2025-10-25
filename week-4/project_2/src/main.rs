use std::io;

fn main() {
    let mut age = String::new();
    println!("What is your age ?");
    io::stdin().read_line(&mut age ).expect("Error");
    let age: u32 = age.trim().parse().expect("Error");

    let mut experience = String::new();
    println!("Do you have experience ? \n yes or no ");
    io::stdin().read_line(&mut experience).expect("Error");
    let exp = experience.trim().to_lowercase();


    if exp == "yes"{

        if age >= 40{
            println!("Your incentive is #1,560,000");
        } else if age >= 30 && age  < 40 {
            println!("Your incentive is #1,480,000");
        } else if age < 28 {
            println!("Your incentive is #1,300,000");
        }; 
    }else if exp == "no"{
        println!("Your incentive is #100,000");
    };



}


