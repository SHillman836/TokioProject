use std::fs::File;

use super::data_pipeline::enums::MathematicalMethod;
use super::data_pipeline::file_manager::file_setup::FileSetup;

/// This sets up the project.
///
/// # Attributes
/// * method (&'a str): mathematical method
/// * file_setup (FileSetup<'a>): instance of the FileSetup class
pub struct Processor<'a> {
    pub method: &'a str,
    pub file_setup: FileSetup<'a>,
}

///This is the implementation for the Processor struct.
impl<'a> Processor<'a> {
    /// This method creates a new  Processor struct.
    ///
    /// # Arguments
    /// * input_filename (&'a str): name of the input file
    /// * method_type (&'a str): mathematical method
    /// * output_filename (&'a str): name of the output file
    ///
    /// # Returns
    /// (FileSetup) The newly created struct
    pub fn new(input_filename: &'a str, method_type: &'a str, output_filename: &'a str) -> Self {
        Processor {
            method: method_type,
            file_setup: FileSetup::new(input_filename, output_filename)
        }
    }

    /// This method matches the string method to the correct MathematicalMethod enum.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (MathematicalMethod) The correct MathematicalMethod enum
    pub fn match_method(&mut self) -> MathematicalMethod {
        match self.method {
            "SQUARE" => MathematicalMethod::SQUARE,
            "ROOT" => MathematicalMethod::ROOT,
            _ => {
                eprintln!("function not supported");
                MathematicalMethod::SQUARE
            }
        }
    }

    /// This method calls the above functions to setup the project.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (MathematicalMethod) The correct MathematicalMethod enum
    pub fn process_input(&mut self) -> MathematicalMethod {
        self.file_setup.setup_input_file();
        self.file_setup.clear_current_output_file();
        self.match_method()
    }
}