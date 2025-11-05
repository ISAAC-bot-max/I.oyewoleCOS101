fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-lekki";
    println!("Name: {}",name);
    println!("University: {},\nAddress: {}",uni,addr);


    let department:&'static str = "ComputerScience";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department,school);
    
    
}
