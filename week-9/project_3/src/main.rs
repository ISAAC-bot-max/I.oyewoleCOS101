use std::fs::File;
use std::io::Write;

fn main() {
    //  Three separate datasets
    let names = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East"
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    
    println!("=== EFCC MERGED DATASET ===");
    for i in 0..names.len() {
        println!(
            "{} |    {}   |    {}",
            names[i], zones[i], ministries[i]
        );
    }

    
    let mut file = File::create("EFCC_Merged_Files.txt").expect("Cannot create file");

    file.write_all(" EFCC MERGED CONVICT DATA\n\n".as_bytes()).unwrap();


    for i in 0..names.len() {
        let line = format!(
            "Name: {}\nZone: {}\nMinistry: {}\n\n",
            names[i], zones[i], ministries[i]
        );
        file.write_all(line.as_bytes()).unwrap();
    }

    println!("EFCC merged data saved successfully!");
}
