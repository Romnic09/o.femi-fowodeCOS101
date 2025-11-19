fn main() {
    let b:(i32, bool, f64)= (10, true, 5.0);
    print(b);
}

fn print(x:(i32, bool, f64)){
    let (age,is_male,cgpa)=x;
    println!("age is {}, Is male? {}, cgpa is {}",age,is_male,cgpa);
}
