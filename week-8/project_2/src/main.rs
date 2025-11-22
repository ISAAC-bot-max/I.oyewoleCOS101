
use std::io;
use std::fs::OpenOptions;
use std::path::Path; 
use std::io::Write;
use std::fs::File;
 use std::io::BufRead;


fn main() {
    loop {
        println!("\n========================================");
        println!("          EXPERIENCE SYSTEM MENU        ");
        println!("========================================");
        println!("1. Candidate: Enter your details");
        println!("2. Manager: Find candidate with highest experience");
        println!("3. Exit program");
        println!("========================================");
        println!("Enter your choice (1/2/3):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => register_candidate(),
            "2" => manager_check(),
            "3" => {
                println!("Exiting program... Goodbye!");
                break;
            }
            _ => println!(" Invalid choice, please enter 1, 2, or 3."),
        }
    }
}


//          USER PART — SAVES DATA TO FILE

fn register_candidate() {
    println!("Candidate Registration Portal");


    let mut name = String::new();
    let mut age_input = String::new();
    let mut exp_input = String::new();

    println!(" Enter your full name:");
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!(" Enter your age:");
    io::stdin().read_line(&mut age_input).unwrap();
    let age: i32 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(" Invalid age. Please enter a number.");
            return;
        }
    };

    println!("Enter your years of programming experience:");
    io::stdin().read_line(&mut exp_input).unwrap();
    let experience: i32 = match exp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(" Invalid experience. Please enter digits only.");
            return;
        }
    };

    // Save details into the manager file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("candidates.txt")
        .unwrap();

    writeln!(file, "{},{},{}", name, age, experience).unwrap();

    println!("\n Your information has been successfully submitted!");
    println!("Thank you for applying, {}!", name);
}



//          MANAGER PART — FINDS BEST CANDIDATE
fn manager_check() {
    
    println!(" Manager Experience Checker");


    let path = "candidates.txt";

    if !Path::new(path).exists() {
        println!(" No candidate data found. File 'candidates.txt' does not exist yet.");
        return;
    }

    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut top_name = String::new();
    let mut top_experience = 0;
    let mut top_age = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 3 {
            continue; // Skip invalid lines
        }

        let name = parts[0].trim();
        let age: i32 = parts[1].trim().parse().unwrap_or(0);
        let experience: i32 = parts[2].trim().parse().unwrap_or(0);

        if experience > top_experience {
            top_name = name.to_string();
            top_age = age;
            top_experience = experience;
        }
    }

    println!("\n Candidate With the Highest Experience:");
    println!(" Name: {}", top_name);
    println!(" Age: {}", top_age);
    println!("Experience: {} years", top_experience);
    println!(" Data processed successfully!");
}
