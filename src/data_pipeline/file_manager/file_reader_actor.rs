use tokio::sync::{mpsc, oneshot, mpsc::Sender};

use std::fs::File;

use super::super::messages::messages::MessageToTransformation

/// This struct defines a file reader actor.
///
/// # Attributes
/// * filename (&str): name of the file to read
/// * sender (Sender<Message>): sender
pub struct FileReaderActor {
    pub filename: &str,
    pub sender: Sender<Message>
}

/// This is the implementation for the FileReaderActor struct.
impl FileReaderActor {
    /// This function creates a new FileReaderActor struct.
    ///
    /// # Arguments
    /// * filename (&str): the name of the file
    /// * sender (Sender<Message>): sender
    ///
    /// # Returns
    /// (FileReaderActor) the newly created struct
    fn new(filename: &str, vector: <Vec<(i32, i32)>, sender: Sender<Message>) -> Self {
        return FileReaderActor {
            filename,
            sender
        }
    }

    /// This function reads a file.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (Vec<u8>) vector of u8 bytes
    fn read_file(self) -> Vec<u8> {
        let mut file = File::open(self.filename.to_string()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
    }

    /// This async function takes in a buffer of bytes, processes those bytes 4 at a time,
    /// and sends them to another actor.
    ///
    /// # Arguments
    /// * buffer (Vec<u8>): vector of u8 bytes
    ///
    /// # Returns
    /// None
    async fn send(self, buffer: Vec<u8>, method_type: MathematicalMethod) {
        /// Create an empty vector and set an index.
        let mut vector: Vec<(i32, i32)> = Vec::new();
        let mut index: i32 = 0;

        /// Loop through the buffer 4 bytes (1 integer)at a time, and push each integer
        /// to the empty vector along with the index.
        for chunk in buffer.chunks_exact(4) {
            vector.clear()
            let value = i32::from_be_bytes(chunk.try_into().expect("Failed to convert slice to i32: Invalid byte sequence"));
            vector.push((index, value));

            /// Open a oneshot channel and send the integer to another actor for processing.
            let (send, recv) = oneshot::channel();
            let message = MessageToTransformation { vector: &mut vector,
                                                    mathematical_method: MathematicalMethod,
                                                    respond_to: send };
            let _ = self.sender.send(message).await;
            match recv.await {
                Err(e) => println!("{}", e),
                Ok(outcome) => println!("here is the outcome of sending to the transformation actor: {}", outcome)
            }
            /// Increment the index.
            index += 1;

            /// Display this error message if the byte multiple is wrong.
            if !buffer.chunks_exact(4).remainder().is_empty() {
                eprintln!("Warning: File Size is not a multiple of 4 bytes")
        }
    }
}

