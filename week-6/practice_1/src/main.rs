fn main() {
    let name="Aisha lawal";
    let uni:&str="Pan atlantic university";
    let addr:&str="Lekki, Lagos";
    println!("Name {}",name);
    println!("University {}, \nAddress: {}",uni,addr);

    let dep:&'static str="Software eng";
    let school:&'static str="School of science and tech";
    println!("Department: {}, \nSchool: {}", dep, school);
}
