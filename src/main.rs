use std::fs;
use day_08_rust::{decode, find_decoder};

fn main() {
    let inputs = fs::read_to_string("input.txt").unwrap();

    let mut counts = 0;
    let mut sum = 0;

    for input in inputs.lines() {

        let mut parts = input.split(" | ");

        let decoder = find_decoder(parts.next().unwrap()).unwrap();

        for (i, token) in parts.next().unwrap().split(" ").enumerate() {
            let decoded_value = decode(token, &decoder).unwrap();
            print!("{:?} ", decoded_value);
            match decoded_value {
                1 => counts += 1,
                4 => counts += 1,
                7 => counts += 1,
                8 => counts += 1,
                _ => {}
            }

            match i {
                0 => sum += decoded_value * 1000,
                1 => sum += decoded_value * 100,
                2 => sum += decoded_value * 10,
                3 => sum += decoded_value * 1,
                _ => {}
            }
        }
        println!();
    }


    println!("counts {}", counts);
    println!("sum {}", sum);

}
