use tokio::sync::{mpsc};

use std::fs;
use std::collections::BTreeMap;

use serde_json::json;

use super::super::messages::messages::MessageToWriter;

/// This defines creates a FileWriterActor struct.
///
/// # Attributes
/// * receiver (mpsc::Receiver<MessageToWriter>): the receiver
/// * filename (&str): the filename to write to
pub struct FileWriterActor<'a> {
    pub receiver: mpsc::Receiver<MessageToWriter>,
    pub filename: &'a str,
}

/// This is the implementation for the FileWriterActor struct.
impl<'a> FileWriterActor<'a> {
    /// This method creates a new FileWriterActor struct.
    ///
    /// # Arguments
    /// * receiver (mpsc::Receiver<MessageToWriter>): the receiver
    /// * filename (&str): the filename to write to
    ///
    /// # Returns
    /// (FileWriterActor) The newly created struct
    pub fn new(receiver: mpsc::Receiver<MessageToWriter>, filename: &'a str) -> Self {
        return FileWriterActor {
            receiver,
            filename
        }
    }

    /// This method takes a message and writes it to a file.
    ///
    /// # Arguments
    /// * message (<MessageToWriter>): the message to write the the file
    ///
    /// # Returns
    /// None
    fn write_to_file(&mut self, message: MessageToWriter) {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        map.insert(message.tuple.0, message.tuple.1);
        let json_data = json!(map);

        fs::write(
            self.filename.to_string(),
            json_data.to_string()).expect("Unable to write to file");

        println!("tuple {:?} written to file", message.tuple);
        let _ = message.respond_to.send(1);
    }

    /// This async method waits until it receives a message and executes when it does
    ///
    /// # Arguments
    /// * message (<MessageToWriter>): the message to write the the file
    ///
    /// # Returns
    /// None
    pub async fn run(mut self) {
        println!("file writer actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.write_to_file(msg);
        }
    }
}
