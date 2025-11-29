
use std::fs::File;
use std::io::Write;

fn main() {
    // Step 1: Student details stored in arrays / vectors
    let names = ["Oluchi Mordi", "Mary Esther", "Samuel Adams","Adekune Gold"];
    let departments = ["Computer Science", "Mass Communication", "Economics","Mechanicall Engineering"];
    let levels = ["100 Level", "200 Level", "300 Level","400 level"];

    // Step 3: Save details into a file
    let mut file = File::create("PAU_Students.txt").expect("Could not create file");

    file.write_all("=== PAU STUDENT INFORMATION SYSTEM ===\n\n".as_bytes()).unwrap();

    for i in 0..names.len() {
        let record = format!(
            "Name: {}\nDepartment: {}\nLevel: {}\n\n",
            names[i], departments[i], levels[i]
        );
        file.write_all(record.as_bytes()).unwrap();
    }

    println!("Student data saved successfully!");
}
