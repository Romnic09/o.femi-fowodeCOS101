fn main() {
   let mut city:Vec<String>=Vec::new();

   println!("the city vector has elements {}",city.len());

   let mut input=String::new();

   println!("How many cities do you want to enter? ");

   std::io::stdin().read_line(&mut input).expect("Failed to read line");
   let size:usize=input.trim().parse().expect("Please type a number!");

   for count in 0..size{
    let mut input2=String::new();
    println!("Enter city name {}: ",count+1);
    std::io::stdin().read_line(&mut input2).expect("Failed to read");
    city.push(input2);
   } 

    println!("The cities you entered are: ");
    let mut count=1;
    for c in city {
        println!("City {}: {}",count,c);
        count+=1;
    }
}
