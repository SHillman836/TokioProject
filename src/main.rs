mod data_pipeline;

use data_pipeline::file_manager::file_reader::FileReader;
use data_pipeline::file_manager::file_writer::FileWriter;
use data_pipeline::mathematical_function::transformation::MathematicalMethod;
use processes::Processor;

use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;

use tokio::main;
use std::time::Instant;
use std::{thread, time};

#[tokio::main]
async fn main() {
    /// Command line processing and file setup
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let function: &String = &args[2].to_uppercase();

    /// Setup and run the processor
    let processor = Processor::new(filename, function);
    let vector = processor.process_input();

    /// Setting up the mpsc channel and cloning the sender
    let (tx, mut rx) = mpsc::channel::<Message>(1);
    let tx_one = tx.clone();

    /// Create 2 tasks
    tokio::spawn(async move {
        let file_reader = FileReader::new(vector, tx_one.clone());
        file_reader.send().await;
        drop(tx_one);
    });
    tokio::spawn(async move {
        let file_reader = FileReader::new(vector, tx.clone());
        file_reader.send().await;
        drop(tx);
    });

    /// Running the FileWriter actor to receive messages
    let output_filename: String = "./output.json".to_string();
    let file_writer = FileWriter::new(rx, output_filename);
    actor.run().await;
}

