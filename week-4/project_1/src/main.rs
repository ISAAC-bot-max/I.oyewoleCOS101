use std::io;// Rust programme codes 

fn main() {
    // Denote the quadrtaic equation 
    println!("The Quadratic equation : Ax^2 + Bx + C");


// storing data of the first value 
    println!("Enter value of A : ");
    let mut a = String::new();
    io::stdin().read_line(&mut  a).expect("Error");
    let value_a:f32 = a.trim().parse().expect("Error");

// collecting data of value B 
    println!("Enter value of B: ");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Error");
    let value_b :f32 = b.trim().parse().expect("Error");

// collecting data of value C 
    println!("Enter value of C");
    let mut c  = String::new();
    io::stdin().read_line(&mut c).expect("Error");
    let value_c:f32 = c.trim().parse().expect("Error");

// finding the discriminant of the inputed data 
    let discriminant = value_b.powf(2.0) -  4.0 * value_a * value_c;

//checking to see wether it has real roots or not 
    if discriminant >= 0.0{
        let r1 : f32 = (-value_b + discriminant.sqrt()) /  (2.0 * value_a);
        let r2 : f32 = (-value_b + discriminant.sqrt()) /  (2.0 * value_a);
        println!("Two distinct real roots {:.2} , {:.2}", r1 , r2);
    }else if discriminant == 0.0{
        let root :f32 = -value_b / (2.0 * value_a);
        println!("One real root: {:.2}", root );
    }else if discriminant <= 0.0 {
        println!("No real roots ");
    };






    
}

