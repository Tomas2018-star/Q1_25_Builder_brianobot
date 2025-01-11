/*
This Utility CLI allows for the conversion of base58 wallet form to bytes arrays and vice versa
*/

use std::io;
use std::io::{stdout, Write};

fn main() {
    println!("+++++++++++++++ Welcome to the Wallet Conversion Utility ++++++++++++++++++");
    loop {

        println!("\nEnter the follow command to choose a utitity: ");
        println!("");
        println!("1. To Convert Base58 wallet to Bytes.");
        println!("2. To Convert bytes to Base58 wallet");
        println!("3. To Close Program\n");

        let mut command = String::new();
        
        io::stdin()
            .read_line(&mut command)
            .expect("Could not read the input from the user");

        let command = command.trim().parse::<u8>().expect("Could not convert input into integer");

        match command {
            1 => {
                print!("Enter the Base58 form here: ");
                stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Invalid Input for Base58");

                let input = input.trim().parse::<String>().expect("Invalid String for Base58");
                println!("Byte Form: {:?}", base58_to_bytes(input))
            },
            2 => {
                print!("Enter the Byte form here: ");
                stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Invalid Input for Bytes");

                let input = input
                    .lines()
                    .next()
                    .unwrap()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim().parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(); 

                println!("Base58 Form: {:?}", bytes_to_base58(input))
            },
            3 => {
                println!("CLosing the CLI program ✨");
                break
            },
            _ => println!("Invalid Input, Please enter a valid number"),
        }
    }
}


fn base58_to_bytes(input: String) -> Vec<u8> {
    bs58::decode(input).into_vec().unwrap()
}


fn bytes_to_base58(input: Vec<u8>) -> String {
    bs58::encode(input).into_string()
}   