fn main() {
    let v = vec!["c","b","f","s","e"];
    
    let mut input1= String::new();  

    println!("Enter index value  ");
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let input1:usize=input1.trim().parse().expect("Please type a number!");

    let ch =v[input1];

    println!("The character at index {} is {}",input1,ch);
}
