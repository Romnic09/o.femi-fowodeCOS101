fn main() {
   let b:(i32.boot.f64)= (10, true, 20.5);
   print(b);

}

fn print(x:(i32, bool, f64)){
    println!("Inside print method");
    println!("{}",x);
}   