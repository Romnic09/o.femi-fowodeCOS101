fn main() {
    let sales = vec![
        ("Toshiba", 2, 450_000.00),
        ("Mac", 1, 1_500_000.00),
        ("HP", 3, 750_000.00),
        ("Dell", 3, 2_850_000.00),
        ("Acer", 1, 250_000.00),
    ];

    let mut ta = 0.0;
    for (_, _, amt) in &sales {
        ta += amt;
    }
    let avg = ta / sales.len() ;

    println!("Total sales amount {:.2}", ta);
    println!("avg sales amount {:.2}", avg);
}
