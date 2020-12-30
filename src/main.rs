use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use glob::glob;
use std::thread;

fn main() {
    // Get the files which have extension .fa
    let entries = glob("data/*.fa").unwrap().filter_map(Result::ok);

    // Create a vector to contain the thread handles to wait for termination
    let mut handles = Vec::new();

    // Loop through the files
    for entry in entries {
        // Display the filename
        println!("{}", entry.display());

        // YUK !!
        // Have to do this to get something concrete to pass to the thread
        // that won't disappear when this loop exits, the chain of function
        // calls makes me feel queasy
        let p = String::from(entry.as_path().to_str().unwrap());

        // Spawn the thread to read the files
        let handle = thread::spawn(|| {
            read_file(p);
        });

        // Add the new handle to the vector
        handles.push(handle)
    }

    // Wait for all the threads to signal completion
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finished");
}

// Function to count the occurances of each base in a thread
fn read_file(filename: String) {
    // Open the file, it will be closed when file goes out of scope
    let file = File::open(filename);

    // Create a buffered reader
    let reader = BufReader::new(file.unwrap());

    // Create a new hashmap to contain the counts
    let mut map = HashMap::new();

    // Read each line in the file
    for line in reader.lines() {
        // Another instance of the ubiquitous and ever mysterious unwrap()...
        let line2 = line.unwrap();

        // Discard lines starting with >
        if !line2.starts_with(">") {
            // Go through each character in the line            
            for ch in line2.chars() {
                // This gets the entry associated with the character
                // or inserts a new key with a value of 0
                let count = map.entry(ch).or_insert(0);

                // Use a dirty, dirty pointer to increment the value associated with the key
                *count += 1;
            }
        }
    }

    // Print the hash map
    println!("{:?}", map)
}
