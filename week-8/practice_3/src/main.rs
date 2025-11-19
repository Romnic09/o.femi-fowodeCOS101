fn value(n: Option<&char>){
    println!("Element of vector {:?}",n);
}

fn main() {
    let v =vec!['r','t','y','u'];

    let mut input1= String::new();
    println!("Enter an index value: ");
    
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let index:usize=input1.trim().parse().expect("Please type a number!");

    let ch: Option<&char> =v.get(index);
    value(ch);
}
