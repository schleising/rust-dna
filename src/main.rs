use std::result::Result::Ok;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use glob::glob;
use std::thread;
use std::path::PathBuf;

fn main() {
    // Get the files which have extension .fa
    let entries = glob("data/*.fa");

    // Shadow entries with the actual value extracted from the result using match
    let entries = match entries {
        // If the result if Ok, extract the actual value from the result wrapper
        Ok(entries) => entries,
        // If the result is Err, print the error and quit, can do nothing more here
        Err(e) => {
            println!("{:?}", e);
            return},
    };

    // Create a vector to contain the thread handles to wait for termination
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    // Loop through the files
    for entry in entries {
        // Shadow entry with the actual value extracted from the result using match
        let entry = match entry {
            Ok(entry) => entry,
            // In this case, if there is an error just break out of the loop and continue with other files
            Err(e) => {
                println!("{:?}", e);
                break},
        };

        // Display the filename
        println!("{}", entry.display());

        // Spawn the thread to read the files
        let handle = thread::spawn(move || {
            read_file(&entry);
        });

        // Add the new handle to the vector
        handles.push(handle)
    }

    // Wait for all the threads to signal completion
    for handle in handles {
        // Again use match to extract the result,
        // the Ok result doesn't seem to be anything useful so use _
        match handle.join() {
            Ok(_) => println!("Thread Terminated"),
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Finished");
}

// Function to count the occurances of each base in a thread
fn read_file(path_buffer: &PathBuf) {
    // Open the file, it will be closed when file goes out of scope
    let file = File::open(path_buffer);

    // Same as before, use pattern macthing to extract the actual file handle
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("{:?}", e);
            return
        }
    };

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Create a new hashmap to contain the counts
    let mut map: HashMap<char, i32> = HashMap::new();

    // Read each line in the file
    for line in reader.lines() {
        match line {
            Ok(line) => parse_line(line, &mut map),
            Err(e) => println!("Error {}", e),
        }
    }

    // Print the hash map
    println!("{:?}", map)
}

fn parse_line(line: std::string::String, map: &mut HashMap<char, i32>) {
    // Discard lines starting with >
    if !line.starts_with(">") {
        // Go through each character in the line            
        for ch in line.chars() {
            // This gets the entry associated with the character
            // or inserts a new key with a value of 0
            let count = map.entry(ch).or_insert(0);

            // Use a dirty, dirty pointer to increment the value associated with the key
            *count += 1;
        }
    }
}
