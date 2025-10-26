use std::io;
fn main() {
    println!("First number ");
    let mut a = String::new();
    println!("Second number ");
    let mut b = String::new();
    println!("Third number ");
    let mut c = String::new();
    io::stdin().read_line(&mut a).expect("failed to read input");
    io::stdin().read_line(&mut b).expect("failed to read input");
    io::stdin().read_line(&mut c).expect("failed to read input");
    let a:f32 = a.trim().parse().expect("please type a number!");
    let b:f32 = b.trim().parse().expect("please type a number!");
    let c:f32 = c.trim().parse().expect("please type a number!");

    let disc:f32 = b*b - 4.0*a*c;
    if disc < 0.0 {
        println!("no real roots");
    } else if disc >0.0{
        let root1:f32 = -b + disc.sqrt() / 2.0*a;
        let root2:f32 = -b - disc.sqrt() / 2.0*a;
        println!("There are 2 roots {} {}",root1,root2);
    
}else if disc==0.0 {
    let root:f32 = -b/2.0*a;
    println!("there is one root {}",root)
}else {
    println!("There is an error")
}


}