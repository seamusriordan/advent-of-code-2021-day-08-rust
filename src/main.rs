use std::fs;
use day_08_rust::{decode, find_decoder};

fn main() {
    // let inputs = fs::read_to_string("example_input.txt").unwrap();
    let inputs = fs::read_to_string("input.txt").unwrap();

    let mut counts = 0;
    for input in inputs.lines() {
        let mut parts = input.split(" | ");

        let decoder = find_decoder(parts.next().unwrap());

        for token in parts.next().unwrap().split(" ") {
            let decoded_value = decode(token, &decoder);
            print!("{:?} ", decoded_value);
            match decoded_value {
                1 => counts += 1,
                4 => counts += 1,
                7 => counts += 1,
                8 => counts += 1,
                _ => {}
            }

        }
        println!();
    }


    println!("counts {}", counts);

}
