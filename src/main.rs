mod file_manager;
mod transformation;
mod enums;

use file_manager::{read_file, write_to_file};
use transformation::Transformation;
use enums::MathematicalMethod;

use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;

#[macro_use] extern crate educe;

fn setup_input_file(filename: &str) {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    match File::create(filename) {
        Ok(mut file) => {
            for &value in &vector {
                file.write_all(&value.to_be_bytes()).unwrap();
            }
        }
        Err(err) => {
            eprintln!("Error creating file: {:?}", err);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let function: &String = &args[2].to_uppercase();
    setup_input_file(filename);

    let filename = format!("./{}", filename);
    let mut vector = read_file(&filename);
    println!("Before operation: {:?}", vector);

    let method = match function.as_str() {
        "SQUARE" => MathematicalMethod::SQUARE,
        "ROOT" => MathematicalMethod::ROOT,
        _ => {
            eprintln!("function not supported");
            MathematicalMethod::SQUARE
        }
    };

    let mut transformation: Transformation = Transformation::new(vector, method.clone());
    match &method {
        &MathematicalMethod::SQUARE => transformation.square(),
        &MathematicalMethod::ROOT => transformation.root(),
        _ => {
            eprintln!("function not supported");
            transformation.root()
        }
    };

    let output_file: String = "./output.json".to_string();
    write_to_file(&output_file, &transformation.vector)
}