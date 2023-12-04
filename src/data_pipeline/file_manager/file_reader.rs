use tokio::sync::{mpsc, oneshot, mpsc::Sender};

use std::fs::File;

use super::message::Message

/// This struct defines a file reader.
///
/// # Attributes
/// * filename (&str): name of the file to read
pub struct FileReader {
    pub filename: &str,
}

/// This is the implementation for the FileReader struct.
impl FileReader {
    /// This function creates a new FileReader struct.
    ///
    /// # Arguments
    /// * filename (&str): the name of the file
    ///
    /// # Returns
    /// (FileReaderActor) the newly created struct
    fn new(filename: &str) -> Self {
        return FileReader {
            filename,
        }
    }
    /// This function reads a file.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (Vec<(i32,i32)>) vector of integers from the file
    fn read_file(self) -> Vec<(i32, i32)> {
        let mut file = File::open(self.filename.to_string()).unwrap();

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).unwrap();

        let mut vector: Vec<(i32, i32)> = Vec::new();
        let mut index: i32 = 0;

        for chunk in buffer.chunks_exact(4) {
            let value = i32::from_be_bytes(chunk.try_into().expect("Failed to convert slice to i32: Invalid byte sequence"));
            vector.push((index, value));
            index += 1;
        }

        if !buffer.chunks_exact(4).remainder().is_empty() {
            eprintln!("Warning: File Size is not a multiple of 4 bytes")
        }

        return vector
    }
}