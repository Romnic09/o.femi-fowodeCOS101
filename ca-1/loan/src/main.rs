use std::io;

fn main() {

let response="y";
 while response.trim() !="n"{
    println!("Input your P value: ");
    let mut p=String::new();
     io::stdin().read_line(&mut p).expect("Error in input");
    let p:f32=p.trim().parse().expect("There is an error");

    println!("Input your R value: ");
    let mut r=String::new();
     io::stdin().read_line(&mut r).expect("Error in input");
    let r:f32=r.trim().parse().expect("There is an error");

    println!("Input your T value: ");
    let mut t=String::new();
     io::stdin().read_line(&mut t).expect("Error in input");
    let t:f32=t.trim().parse().expect("There is an error");
    
    
    let a:f32=p*(1.0+(r/100.0_f32)).powf(t);
    let interest:f32=a - p;

    println!("The amount after {} years is {} and the interest is {}", t,a,interest);

    println!("Do you want to calculate another broker (y/n): ");
    let mut response=String::new();
    io::stdin().read_line(&mut response).expect("Error in input");

}




}
