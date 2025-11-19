fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("Array contents: {:?}", numbers);

    let slice1=&numbers[1..3];
    println!("Slice 1: {:?}", slice1);

    let slice2=&numbers[..3];
    println!("Slice 2: {:?}", slice2);

    let slice3=&numbers[2..];
    println!("Slice 3: {:?}", slice3);

    let slice4= &numbers[..];
    println!("Slice 4: {:?}", slice4);

}
