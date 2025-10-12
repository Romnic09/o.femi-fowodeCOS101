fn main() {
    let prin: f64 = 520_000_000.0; 
    let rt: f64 = 10.0; 
    let tm: f64 = 5.0; 

    let amt= prin * (1.0 + rate / 100.0).powf(tm);
    let ci= amt - prin;

    println!("Total amount after 5 years is ₦{:.2}", amt);
    println!("Compound interest is: ₦{:.2}", ci);
}