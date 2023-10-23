use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Attempt to create a CSV reader from the given file path.
    let mut csv_reader = match csv::Reader::from_path(path) {
        Ok(csv_reader) => csv_reader, // Successfully created the CSV reader.
        Err(err_msg) => return Err(err_msg.into()), // Handle the error and convert it to a Result.
    };

    //let mut csv_reader = csv::Reader::from_path(path) ? concise match statement !!
    /*
     */
    // Iterate over each record in the CSV file and print it.
    for result in csv_reader.records() {
        // Handle the result, which can be an Ok() or an Err() containing an error.
        let record = result?;
        println!("{:?}", record);
    }

    Ok(()) // Return Ok(()) to indicate success.
}

fn main() {
    if let Err(err_msg) = read_from_file("../employees.csv") {
        // ../employees.csv  since csv file is out of src folder 
        // Handle any error that might occur during file reading and printing.
        eprintln!("{}", err_msg);
    }
}

