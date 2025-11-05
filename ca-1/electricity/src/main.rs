use std::io;

fn main() {
    println!("Enter your name: ");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("Error in input");

    println!("Enter the units consumed: ");
    let mut units=String::new();
    io::stdin().read_line(&mut units).expect("Error in input");
    let units:f64=units.trim().parse().expect("There is an error");

    let mut price:f64=0.0;
    let mut rate:f64=0.0;

    if units >= 0.0 && units <= 100.0{
        rate=20.0;
        price = 20.0*units;
    } else if units>=101.0 && units<=300.0{
        rate=35.0;
        price =35.0*price;

    } else if units >=301.0 && units<=500.0{
        rate=50.0;
        price=50.0*price;
    }else{
        println!("Extra 5000 naira charged");
        rate=50.0; 
        price=50.0*price +5000.0;
    }

    println!("{} the number of units used is {} , the rate is {} and the total price is {} ",name,units,rate,price);

}
