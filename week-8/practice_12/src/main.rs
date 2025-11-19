fn main() {
    let mut colors =["red","green","blue","yellow"];
    println!("Original colors: {:?}",colors);

    let sliced= &mut colors[1..3];

    println!("Sliced colors: {:?}",sliced);

    sliced[1]="purple";

    println!("Updated colors: {:?}",sliced);
}
