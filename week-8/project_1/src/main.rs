use std::io;

fn main() {
    let office_admin=vec!["Intern","Administrator","Senior administrator","office manager","director","CEO"];
    let academic=vec!["","Research assistant","PHD candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let lawyer=vec!["Paralegal","Junior Associate","Associate","Senior associate 1-2","Senior Associate 3-4","Partner"];
    let teacher=vec!["Placement","Classroom teacher","Snr teacher","Leading Teacher","Deputy Principal","Principal"];

    println!("Enter your profession (office/academic/lawyer/teacher):");
    let mut profession=String::new();
    io::stdin().read_line(&mut profession).expect("Failed to read line");
    println!("Enter the number of years of experience:");
    let mut experience=String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experience:usize=experience.trim().parse().expect("Please type a number!");

    if profession.trim()=="office"{

    if experience>=1 && experience<=2 {
        println!("you are an {}",office_admin[0]);
    }
    else if  experience>=3 && experience<5 {
        println!("you are an {}",office_admin[1]);

    }else if  experience>=5 && experience<8{
        println!("you are a {}",office_admin[2]);

    }else if  experience>=8 && experience<10{
        println!("you are an {}", office_admin[3]);
    }else if  experience>=10 && experience<=13{
        println!("you are a {}",office_admin[4]);
    }else {
        println!("you are a {}",office_admin[5]);
    }

    }


    if profession.trim()=="academic"{

    if experience>=1 && experience<=2 {
        println!("you are an {}",academic[0]);
    }
    else if  experience>=3 && experience<5 {
        println!("you are an {}",academic[1]);

    }else if  experience>=5 && experience<8{
        println!("you are a {}",academic[2]);

    }else if  experience>=8 && experience<10{
        println!("you are an {}", academic[3]);
    }else if  experience>=10 && experience<=13{
        println!("you are a {}",academic[4]);
    }else {
        println!("you are a {}",academic[5]);
    }

    }

    if profession.trim()=="lawyer"{

    if experience>=1 && experience<=2 {
        println!("you are an {}",lawyer[0]);
    }
    else if  experience>=3 && experience<5 {
        println!("you are an {}",lawyer[1]);

    }else if  experience>=5 && experience<8{
        println!("you are a {}",lawyer[2]);

    }else if  experience>=8 && experience<10{
        println!("you are an {}", lawyer[3]);
    }else if  experience>=10 && experience<=13{
        println!("you are a {}",lawyer[4]);
    }else {
        println!("you are a {}",lawyer[5]);
    }

    }

    if profession.trim()=="teacher"{

    if experience>=1 && experience<=2 {
        println!("you are an {}",teacher[0]);
    }
    else if  experience>=3 && experience<5 {
        println!("you are an {}",teacher[1]);

    }else if  experience>=5 && experience<8{
        println!("you are a {}",teacher[2]);

    }else if  experience>=8 && experience<10{
        println!("you are an {}", teacher[3]);
    }else if  experience>=10 && experience<=13{
        println!("you are a {}",teacher[4]);
    }else {
        println!("you are a {}",teacher[5]);
    }

    }
}
