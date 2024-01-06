use tokio::sync::{oneshot, mpsc::Sender};

use std::fs::File;
use std::io::Read;

use super::router_actor::RouterActor;
use super::super::super::messages::messages::MessageToTransformation;
use super::super::super::enums::MathematicalMethod;

/// This struct defines a file reader actor.
///
/// # Attributes
/// * filename (&'a str): name of the file to read
/// * router (RouterActor): router
pub struct FileReaderActor<'a> {
    pub filename: &'a str,
    pub router: RouterActor,
}

/// This is the implementation for the FileReaderActor struct.
impl<'a> FileReaderActor<'a> {
    /// This function creates a new FileReaderActor struct.
    ///
    /// # Arguments
    /// * filename (&str): the name of the file
    /// * router (RouterActor): router
    ///
    /// # Returns
    /// (FileReaderActor) the newly created struct
    pub fn new(filename: &'a str, router: RouterActor) -> Self {
        return FileReaderActor {
            filename,
            router
        }
    }

    /// This function reads a file.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (Vec<u8>) vector of u8 bytes
    pub fn read_file(&mut self) -> Vec<u8> {
        let mut file = File::open(self.filename.to_string()).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
    }

    /// This async function takes in a buffer of bytes, processes those bytes 4 at a time,
    /// and sends them to another actor.
    ///
    /// # Arguments
    /// * buffer (Vec<u8>): vector of u8 bytes
    /// * method_type (MathematicalMethod): mathematical method
    ///
    /// # Returns
    /// None
    pub async fn send(&mut self, buffer: Vec<u8>, method_type: MathematicalMethod) {
        // Set an index.
        let mut index: i32 = 1;

        // Loop through the buffer 4 bytes (1 integer) at a time, and push each integer
        // to the empty vector along with the index.
        for chunk in buffer.chunks_exact(4) {
            if let Ok(value) = chunk.try_into().map(i32::from_be_bytes) {
                println!("Sending tuple of index: {}, value: {} to router", index, value);
                let message = MessageToTransformation {
                    tuple: (index, value),
                    mathematical_method: method_type.clone(),
                };
                self.router.route(message).await;
            } else {
                eprintln!("Failed to convert slice to i32: Invalid byte sequence");
            }
            index += 1;
        }

        if !buffer.chunks_exact(4).remainder().is_empty() {
            eprintln!("Warning: File Size is not a multiple of 4 bytes");
        }
    }
}

