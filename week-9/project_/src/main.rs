use std::fs::File;
use std::io::Write;

fn main(){
    //collecting the data ijn arrays 
    let mut file = File::create("Nigerian_Beweries_Plc.txt").unwrap();
    let mut file = std::fs::OpenOptions::new().append(true).open("Nigerian_Beweries_Plc.txt").unwrap();
    let larger = ["Lager","33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout = ["Legend","Turbo King","Williams"];
    let non_all = ["Matina","Amstel Malta","Malta Gold","Fayrouz"];

    file.write_all("Nigerian Breweries PLC (Drinks)".as_bytes()).unwrap();
    file.write_all("--------------------------------".as_bytes()).unwrap();

         // Simple table format

    file.write_all("\n\n=== Nigerian Breweries Product Table ===\n".as_bytes()).unwrap();
    file.write_all("----------------------------------------\n".as_bytes()).unwrap();

    // LAGER row
    let lager_row = format!("Lager:\t\t{}\n", larger.join(", "));
    file.write_all(lager_row.as_bytes()).unwrap();

    // STOUT row
    let stout_row = format!("Stout:\t\t{}\n", stout.join(", "));
    file.write_all(stout_row.as_bytes()).unwrap();

    // NON-ALCOHOLIC row
    let non_all_row = format!("Non-Alc:\t{}\n", non_all.join(", "));
    file.write_all(non_all_row.as_bytes()).unwrap();


}
