use std::fs::File;
use std::io::Write;
use std::io::Read;

/// This struct sets up a file.
///
/// # Attributes
/// * input_filename (&'a str): name of the input file
/// * output_filename (&'a str): name of the output file
pub struct FileSetup<'a> {
    pub input_filename: &'a str,
    pub output_filename: &'a str,
}

///This is the implementation for the FileSetup struct.
impl<'a> FileSetup<'a> {
    /// This method creates a new  FileSetup struct.
    ///
    /// # Arguments
    /// * input_filename (&'a str): name of the input file
    /// * output_filename (&'a str): name of the output file
    ///
    /// # Returns
    /// (FileSetup) The newly created struct
    pub fn new(input_filename: &'a str, output_filename: &'a str) -> Self {
        return FileSetup {
            input_filename,
            output_filename,
        }
    }

    /// This method sets up the file.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub fn setup_input_file(&mut self) {
        let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        match File::create(self.input_filename) {
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

    /// This clears an output file.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub fn clear_current_output_file(&mut self) {
        match File::create(self.output_filename) {
                Ok(_) => println!("File contents cleared."),
                Err(e) => eprintln!("Failed to clear file contents: {}", e),
            }
    }
}