use std::io;

fn main() {
    println!("\nStuent information management system");

    println!("\nPlease enter your name");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Your name is {}",name);




}
