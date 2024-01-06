use super::data_pipeline::enums::MathematicalMethod;
use super::data_pipeline::file_manager::file_setup::FileSetup;

pub struct Processor<'a> {
    pub filename: &'a str,
    pub method: &'a str,
}

impl<'a> Processor<'a> {
    pub fn new(file_name: &'a str, method_type: &'a str) -> Self {
        Processor {
            filename: file_name,
            method: method_type,
        }
    }

    pub fn setup_file(&mut self) {
        FileSetup::new(self.filename).setup_input_file();
    }

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

    pub fn process_input(&mut self) -> MathematicalMethod {
        let mut processor = Processor::new(self.filename, self.method);
        processor.setup_file();
        processor.match_method()
    }
}