use tokio::sync::{mpsc, oneshot, mpsc::Sender};

use std::fs::File;
use std::fs;
use std::io::Read;
use std::collections::BTreeMap;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use super::message::Message

/// This defines creates a FileWriter struct.
///
/// # Attributes
/// * receiver (mpsc::Receiver<Message>): the receiver
/// * filename (&str): the filename to write to
pub struct FileWriter {
    pub receiver: mpsc::Receiver<Message>,
    pub filename: &str,
}

/// This is the implementation for the FileWriter struct.
impl FileWriter {
    /// This method creates a new FileWriter struct.
    ///
    /// # Arguments
    /// * receiver (mpsc::Receiver<Message>): the receiver
    /// * filename (&str): the filename to write to
    ///
    /// # Returns
    /// (FileWriter) The newly created struct
    fn new(receiver: mpsc::Receiver<Message>, filename: &str) -> Self {
        return FileWriter {
            receiver,
            filename
        }
    }

    /// This method takes a message and writes it to a file.
    ///
    /// # Arguments
    /// * message (Message>): the message to write the the file
    ///
    /// # Returns
    /// None
    fn write_to_file(self, message: Message) {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        for (key, value) in message.vector {
            map.insert(*key, *value);
        }

        let json_data = json!(map);
        fs::write(
            self.filename.to_string(),
            json_data.to_string()).expect("Unable to write to file")

        println!("vector {} sent", message.vector)
        let _ = message.respond_to.send(1);
    }

    /// This async method waits until it receives a message and executes when it does
    ///
    /// # Arguments
    /// * message (<Message>): the message to write the the file
    ///
    /// # Returns
    /// None
    async fn run(mut self) {
        println!("actor is running");
        while let Some(msg) = self.receiver.recv().await {
            self.write_to_file(msg);
        }
    }
}
