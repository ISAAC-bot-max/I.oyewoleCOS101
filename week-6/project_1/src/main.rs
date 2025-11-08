use std::io;

fn main(){
    let mut greeting = String::from("Welcome");
    greeting.push_str("to Ordering");
    println!("{}",greeting);


    
    println!("codes  -----menu---               \tprices");
    println!("P \tPoundo Yam/Edinkaiko Soup      \t3,200");
    println!("F \tFried Rice & Chicken           \t3,000");
    println!("A \tAmala & Ewedu Soup            \t2,500");
    println!("E \tEba & Egusi Soup               \t 2,000");
    println!("W \tWhite Rice & Stew              \t 2,500");

    println!("input the code of food here. (P/F/A/E/W) ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_uppercase();


   

    println!("How many do you want? ");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:f32 = quantity.trim().parse().expect("Failed to read input");


    let price:f32 = match input.as_str(){
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,

        _ => {
            println!("order complete");
            return;
        }

    };

    let total:f32 = price * quantity;
    let discount:f32 = total -  (total * (5.0 / 100.0));

    if total > 10_000.0{
        let mut choice = String::new();
        println!("Do you want a discount of 5% with that .. (y/n)");
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().to_lowercase();


        if choice == "y"{
            println!("the total cost is {:.2} with discount",discount);
        }else if choice == "n"{
            println!("The total cost is {}",total);
        }

    }else{
        println!("the total cost is {}:",total);
    }



        



    

}