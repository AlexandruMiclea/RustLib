use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn caesar(text: &str, shift: i32) -> String{
    // make sure that the shift is in 0-25 range
    let shift: u8 = (shift % 26) as u8;
    let mut ret: String = String::new();

    for ch in text.chars() {
        let mut val: u8 = ch as u8;
        // uppercase letter, apply caesar
        if 65 <= val && val <= 90 {
            val = (val - 65 + shift) % 26 + 65;
        }
        // lowercase letter, apply caesar
        else if 97 <= val && val <= 122 {
            val = (val - 97 + shift) % 26 + 97;
        }
        // digit, apply caesar 
        else if 48 <= val && val <= 57 {
            val = (val - 48 + shift) % 10 + 48
        }
        // all other characters remain intact
        let new_ch: char = val as char;

        ret.push(new_ch);
    }
    ret.push('\n');
    ret
}

fn main() {
    // how to use: 
    // cargo run -- filename shift
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    println!("{argc}");
    println!("{}", argv[0]);
    assert_eq!(argc, 3);

    let mut in_file = String::from("src/");
    let file_name = String::from(&argv[1]);
    in_file.push_str(&file_name);
    println!("{in_file}");
    let shift_val: i32 = argv[2].trim().parse().expect("Second parameter should be a number!");

    let mut file = File::open(in_file).expect("File not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read contents from input file!");

    let rows: Vec<&str> = contents.split('\n').collect();

    // create_new is unstable as of writing this code
    let mut out_file = File::options().read(true).write(true).create(true)
        .open("src/encrypted.txt")
        .expect("Unexpected error when creating output file!");

    for row in rows {
        out_file.write(caesar(row, shift_val).as_bytes()).expect("Problem when writing to output file!");
    }
    

}