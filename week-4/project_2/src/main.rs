use std::io;

fn main() {
    let mut input = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input).expect("Eror");
    let experience = input.trim().to_lowercase();
    input.clear();

    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut input).expect("Eror");
    let age: u32 = input.trim().parse().expect("Eror");

    let incentive: f64;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000.0;
        } else if age >= 30 {
            incentive = 1_480_000.0;
        } else if age < 28 {
            incentive = 1_300_000.0 * 12.0; 
        } else {
            incentive = 1_000_000.0; 
        }
    } else {
        incentive = 100_000.0;
    }

    println!("The annual incentive for the employee is ₦{:.2}", incentive);
}
