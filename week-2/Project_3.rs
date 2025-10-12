fn main() {
    let prin: f64 = 510_000.0;  
    let rate: f64 = 5.0;             
    let yr: u32 = 3;              

    let val = principal * (1.0 - (rate / 100.0)).powi(yr as i32);

    println!(" value of the tv after {} years is {:.2}", yr, val);
}
