use std::io;

// This function reads a number from the user and converts it to f64
fn get_number(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let number: f64 = input.trim().parse().expect("Please enter a valid number");
    number
}

fn main() {
    loop{
        println!("RUST CALCULATION PROGRAM");
        println!("-------------------------");

        println!("Choose a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");



        println!("Enter any code (1--5)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = choice.trim().parse().expect("Invalid choice");

        let  _code = match choice {
            1 => area_trapezium(),
            2 => area_rhombus(),
            3 =>area_parallelogram(),
            4 =>area_cube(),
            5 =>volume_cylinder(),
            __=> {println!(" Invalid choice");
            break ;}
        };
    };
}
  

   // 1. Trapezium
fn area_trapezium() {
    let height = get_number("Enter height:");
    let base1 = get_number("Enter base 1:");
    let base2 = get_number("Enter base 2:");

    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}

// 2. Rhombus
fn area_rhombus() {
    let d1 = get_number("Enter diagonal 1:");
    let d2 = get_number("Enter diagonal 2:");

    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

// 3. Parallelogram
fn area_parallelogram() {
    let base = get_number("Enter base:");
    let height = get_number("Enter height:");

    let area = base * height;
    println!("Area of Parallelogram = {}", area);
}

// 4. Cube
fn area_cube() {
    let side =get_number("Enter length of the side:");

    let area = 6.0 * side * side;
    println!("Area of Cube = {}", area);
}

// 5. Cylinder
fn volume_cylinder() {
    let radius = get_number("Enter radius:");
    let height = get_number("Enter height:");

    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume of Cylinder = {}", volume);
}
