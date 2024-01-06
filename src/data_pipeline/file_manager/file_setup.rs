use std::fs::File;
use std::io::Write;
use std::io::Read;

/// This struct sets up a file.
///
/// # Attributes
/// * filename (&str): name of the file to setup
pub struct FileSetup<'a> {
    pub filename: &'a str
}

///This is the implementation for the FileSetup struct.
impl<'a> FileSetup<'a> {
    /// This method creates a new  FileReader struct.
    ///
    /// # Arguments
    /// * filename (&str): name of the file to read
    ///
    /// # Returns
    /// (FileSetup) The newly created struct
    pub fn new(filename: &'a str) -> Self {
        return FileSetup {
            filename
        }
    }

    /// This method sets up the file.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub fn setup_input_file(self) {
        let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        match File::create(self.filename) {
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
}