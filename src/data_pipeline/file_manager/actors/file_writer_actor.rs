use tokio::sync::{mpsc};
use std::fs::{self, OpenOptions};
use std::io::{self, Read};
use std::collections::BTreeMap;
use serde_json::{json, Value, Map};

use super::super::super::messages::messages::MessageToWriter;

/// This defines creates a FileWriterActor struct.
///
/// # Attributes
/// * receiver (mpsc::Receiver<MessageToWriter>): the receiver
/// * filename (&'a str): the filename to write to
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
    /// * filename (&'a str): the filename to write to
    ///
    /// # Returns
    /// (FileWriterActor) The newly created struct
    pub fn new(receiver: mpsc::Receiver<MessageToWriter>, filename: &'a str) -> Self {
        return FileWriterActor {
            receiver,
            filename
        }
    }

    /// This method reads the existing content in a file.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (io::Result<BTreeMap<i32, i32>>) The existing content
    fn read_existing_content(&self) -> io::Result<BTreeMap<i32, i32>> {
        let mut file = fs::File::open(self.filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if content.is_empty() {
            Ok(BTreeMap::new()) // Return an empty map if the file is empty
        } else {
            // Parse the existing JSON into a BTreeMap
            let existing_data: BTreeMap<i32, i32> = serde_json::from_str(&content)?;
            Ok(existing_data)
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
        // Read the existing content into a BTreeMap
        let mut map = match self.read_existing_content() {
            Ok(m) => m,
            Err(_) => {
                println!("Failed to read existing file content. Creating a new file.");
                BTreeMap::new()
            }
        };

        // Insert the new data into the map
        map.insert(message.tuple.0, message.tuple.1);

        // Convert the entire map to JSON
        let json_data = json!(map);

        // Write the updated JSON back to the file, overwriting the old content
        fs::write(self.filename, json_data.to_string()).expect("Unable to write to file");

        println!("Updated tuple {:?} written to file", message.tuple);
        let _ = message.respond_to.send(1);
    }


    /// This async method waits until it receives a message and executes when it does
    ///
    /// # Arguments
    /// None
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
