fn main() {
    let mut mountain=("Everest",8848,"Fishtail",6993);
    
    println!("Original tuple = {}",mountain);

    mountain.1=8850;
    mountain.3=6995;

    println!("Updated tuple = {}",mountain);
}
