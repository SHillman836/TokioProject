mod data_pipeline;
mod processes;

use data_pipeline::file_manager::actors::file_reader_actor::FileReaderActor;
use data_pipeline::file_manager::actors::transformation_actor::TransformationActor;
use data_pipeline::file_manager::actors::file_writer_actor::FileWriterActor;
use data_pipeline::file_manager::actors::router_actor::RouterActor;
use data_pipeline::enums::MathematicalMethod;

use data_pipeline::messages::messages::MessageToTransformation;
use data_pipeline::messages::messages::MessageToWriter;

use processes::Processor;

use std::env;

use tokio::sync::{mpsc};

#[tokio::main]
async fn main() {
    // Command line processing and file setup
    let args: Vec<String> = env::args().collect();
    let filename: String = args[1].clone();
    let function: String = args[2].to_uppercase().clone();

    // Setup and run the processor
    let mut processor = Processor::new(&filename, &function);
    let method = processor.process_input();

    // Create senders for each TransformationActor
    let (tx_transformation1, rx_transformation1) = mpsc::channel::<MessageToTransformation>(5);
    let (tx_transformation2, rx_transformation2) = mpsc::channel::<MessageToTransformation>(5);

    // Create and set up the router
    let router = RouterActor::new(vec![tx_transformation1, tx_transformation2]);

    // Create a channel for communication with FileWriterActor
    let (tx_writer, rx_writer) = mpsc::channel::<MessageToWriter>(5);
    let tx_writer_clone_1 = tx_writer.clone();
    let tx_writer_clone_2 = tx_writer.clone();

    // Set up FileReaderActor task and pass in the router
    tokio::spawn(async move {
        let mut file_reader = FileReaderActor::new(&filename, router);
        let buffer = file_reader.read_file();
        file_reader.send(buffer, method).await;
    });

    // Set up TransformationActor tasks
    tokio::spawn(async move {
        let transformation_actor = TransformationActor::new(rx_transformation1, tx_writer_clone_1);
        transformation_actor.run().await;
    });

    tokio::spawn(async move {
        let transformation_actor = TransformationActor::new(rx_transformation2, tx_writer_clone_2);
        transformation_actor.run().await;
    });

    // Running the FileWriter actor to receive messages
    let output_filename: String = "./output.json".to_string();
    let file_writer = FileWriterActor::new(rx_writer, &output_filename);
    file_writer.run().await;
}

