use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Drink categories
    let lager = [
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = [
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = [
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create output file
    let mut file = File::create("drinks.txt")?;

    // Write header
    writeln!(
        file,
        "{:<20} {:<20} {:<20}",
        "Lager", "Stout", "Non-Alcoholic"
    )?;
    writeln!(file, "{}", "-".repeat(60))?;

    // Determine longest column
    let max_len = lager.len().max(stout.len()).max(non_alcoholic.len());

    // Write rows
    for i in 0..max_len {
        let lager_item = lager.get(i).unwrap_or(&"");
        let stout_item = stout.get(i).unwrap_or(&"");
        let non_alc_item = non_alcoholic.get(i).unwrap_or(&"");

        writeln!(
            file,
            "{:<20} {:<20} {:<20}",
            lager_item, stout_item, non_alc_item
        )?;
    }

    println!("File 'drinks.txt' has been created successfully.");
    Ok(())
}
