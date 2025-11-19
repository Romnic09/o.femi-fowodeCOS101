fn main() {
    let tuple:(&str, f32, u8)=("Rust",3.14,100);
    println!("Tuple content is {}", tuple);

    let tuple2=("Rust","fun",100);
    println!("tuple contents {}", tuple);

    println!("First element is {}", tuple.0);
    println!("Second element is {}", tuple.1);
    println!("Third element is {}", tuple.2);
    
}
