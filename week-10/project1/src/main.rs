struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn cost_for(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { brand: "HP".into(), price: 650_000 };
    let ibm = Laptop { brand: "IBM".into(), price: 755_000 };
    let toshiba = Laptop { brand: "Toshiba".into(), price: 550_000 };
    let dell = Laptop { brand: "Dell".into(), price: 850_000 };

    let quantity = 3;

    let total_cost =
        hp.cost_for(quantity) +
        ibm.cost_for(quantity) +
        toshiba.cost_for(quantity) +
        dell.cost_for(quantity);

    println!("Total cost for 3 units of each brand is: ₦{}", total_cost);
}
