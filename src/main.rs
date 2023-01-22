use csv;
use std::error::Error;

fn main() {

    // Set csv file path
    let path: &str = "./csv_files/heart_failure_clinical_records_dataset.csv";

    // Create a function that's recive as param a csv file path && return as result file records || Error exception
    fn read_csv_file(path: &str) -> Result<(), Box<dyn Error>> {

        // Using crate csv module for read from path the file
        let mut reader = csv::Reader::from_path(path)?;

        // Iterating the content csv file and return it as string with .records(), saving the results at the record variable
        for result in reader.records() {
            let record = result?;

            println!("{:?}", record)
        }
        Ok(())
    }

    // Throwing error exception
    if let Err(e) = read_csv_file(path) {
        eprintln!("{}", e);
    }
} 