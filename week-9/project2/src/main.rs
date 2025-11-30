use std::fs::File;
use std::io::{self, Write};

struct Student {
    name: String,
    matric: String,
    department: String,
    score: u32,
}

fn main() -> io::Result<()> {
    
    let students = vec![
        Student { 
            name: "Oluchi Mordi".to_string(),
            matric: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            score: 300,
        },
        Student { 
            name: "Adams Aliyu".to_string(),
            matric: "ECO10011001".to_string(),
            department: "Economics".to_string(),
            score: 100,
        },
        Student { 
            name: "Shania Bolade".to_string(),
            matric: "CSC10328828".to_string(),
            department: "Computer Science".to_string(),
            score: 200,
        },
        Student { 
            name: "Adekunle Gold".to_string(),
            matric: "EEE10200202".to_string(),
            department: "Electrical".to_string(),
            score: 400,
        },
        Student { 
            name: "Blanca Edemoh".to_string(),
            matric: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            score: 100,
        },
    ];

    println!("{:^80}", "PAU SMIS");
    println!(
        "{:<20} {:<20} {:<20} {:<10}",
        "Student Name", "Matric Number", "Department", "Score"
    );
    println!("{}", "-".repeat(70));

    for s in &students {
        println!(
            "{:<20} {:<20} {:<20} {:<10}",
            s.name, s.matric, s.department, s.score
        );
    }

    let mut file = File::create("students.txt")?;

    writeln!(file, "{:^80}", "PAU SMIS")?;
    writeln!(
        file,
        "{:<20} {:<20} {:<20} {:<10}",
        "Student Name", "Matric Number", "Department", "Score"
    )?;
    writeln!(file, "{}", "-".repeat(70))?;

    for s in students {
        writeln!(
            file,
            "{:<20} {:<20} {:<20} {:<10}",
            s.name, s.matric, s.department, s.score
        )?;
    }

    println!("\n✔ File 'students.txt' has been created successfully.");

    Ok(())
}
