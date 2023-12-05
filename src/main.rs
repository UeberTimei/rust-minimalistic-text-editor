use std::{io::{self, Read, Write}, fs::{OpenOptions, File}};

fn main() -> Result<(), io::Error> {
    let mut file = OpenOptions::new().read(true).open("hello.txt").expect("There is no such file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    drop(file);

    println!("Content: \n{}\n", buffer);

    let mut buf_num = String::new();
    let num = match io::stdin().read_line(&mut buf_num) {
        Ok(_) => buf_num.trim().parse::<usize>().unwrap(),
        Err(_) => 0,
    };
    let mut new_data: Vec<String> = buffer.lines().map(|line| line.trim().to_string()).collect();

    let mut change_to = String::new();
    io::stdin().read_line(&mut change_to).expect("An error occurred while reading new data");
    new_data[num] = change_to.trim().to_string();

    let mut changed_file = File::create("hello.txt").unwrap();
    changed_file.write(new_data.join("\n").as_bytes()).expect("An error occured while writing new data");
    drop(changed_file);

    Ok(())
}
