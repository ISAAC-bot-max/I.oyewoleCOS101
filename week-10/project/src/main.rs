struct Laptop {
    brand: String,
    price: i32,
}

impl Laptop {
    // method to calculate cost of buying n units
    fn cost(&self, quantity: i32) -> i32 {
        self.price * quantity
    }
}

fn main() {
    // creating objects
    let hp = Laptop { brand: "HP".to_string(), price: 650000 };
    let ibm = Laptop { brand: "IBM".to_string(), price: 755000 };
    let tosh = Laptop { brand: "Toshiba".to_string(), price: 550000 };
    let dell = Laptop { brand: "Dell".to_string(), price: 850000 };

    // quantity to buy
    let qty = 3;


    let total_cost = hp.cost(qty) + ibm.cost(qty) + tosh.cost(qty)+ dell.cost(qty);

    println!("Total cost for purchasing 3 of each brand is: {}", total_cost);
}
