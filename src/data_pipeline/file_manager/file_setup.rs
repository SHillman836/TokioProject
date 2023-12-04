use std::fs::File;

/// This struct sets up a file.
///
/// # Attributes
/// * filename (&str): name of the file to setup
pub struct FileSetup {
    pub filename: &str
}

///This is the implementation for the FileSetup struct.
impl FileSetup {
    /// This method creates a new  FileReader struct.
    ///
    /// # Arguments
    /// * filename (&str): name of the file to read
    ///
    /// # Returns
    /// (FileSetup) The newly created struct
    fn new(filename: &str) -> self {
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
    fn setup_input_file(self) {
        let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

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
}