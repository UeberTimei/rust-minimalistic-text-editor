use std::{io::{self, Read, Write}, fs::{OpenOptions, File}};

fn main() -> Result<(), io::Error> {
    // Open the file in read mode
    let mut file = OpenOptions::new().read(true).open("hello.txt").expect("There is no such file");

    // Read the file content into a string buffer
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    drop(file); // Close the file

    // Print the content of the file
    println!("Content: \n{}\n", buffer);

    // Read user input for line number to modify
    let mut buf_num = String::new();
    let num = match io::stdin().read_line(&mut buf_num) {
        Ok(_) => buf_num.trim().parse::<usize>().unwrap(),
        Err(_) => 0,
    };

    // Create a vector to hold each line of the file
    let mut new_data: Vec<String> = buffer.lines().map(|line| line.trim().to_string()).collect();

    // Read user input for the new data to replace the specified line
    let mut change_to = String::new();
    io::stdin().read_line(&mut change_to).expect("An error occurred while reading new data");

    // Update the specified line with the new data
    new_data[num] = change_to.trim().to_string();

    // Create a new file or overwrite existing file with the modified data
    let mut changed_file = File::create("hello.txt").unwrap();
    changed_file.write(new_data.join("\n").as_bytes()).expect("An error occurred while writing new data");
    drop(changed_file); // Close the file

    Ok(())
}
