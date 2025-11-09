use std::io;

fn main() {
    println!("P=Pounded yam- 3200");
    println!("F=Fried rice and chicken- 3000");
    println!("A=Amala and ewedu soup- 2500");
    println!("E=Eba and egusi- 2000");
    println!("W=White rice and stew- 2500");
let mut total:f32=0.0;
    println!("Enter the food code you want to order: ");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Enter the quantity you want to order: ");

    let mut quan=String::new();
    io::stdin().read_line(&mut quan).expect("Failed to read line");
    let quan:f32=quan.trim().parse().expect("Please type a number!");

    

    if name.trim()=="P"{
        total= quan*3200.0;
}else if name.trim()=="F"{
    total=quan*3000.0;

}else if name.trim()=="A"{
    total=quan*2500.0;
}else if name.trim() == "E"{
    total=quan*2000.0;
}else if name.trim()=="W"{
    total=quan*2500.0;



}else{
    println!("There is no such code");

}

if total>10000.0{
    total=total - (total*0.05);
}
println!("Total is: {}",total);

}