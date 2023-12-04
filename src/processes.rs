use super::data_pipeline::enums::MathematicalMethod;
use super::data_pipeline::mathematical_function::transformation;
use super::data_pipeline::file_manager::file_reader::FileReader;
use super::data_pipeline::file_manager::file_setup::FileSetup;

pub struct Processor {
    pub filename: &String,
    pub method: &String,
    pub vector: Option<Vec<(i32, i32)>>
}

impl Processor {
    pub fn new(file_name: &String, method_type: &String) -> Self {
        return Processor {
            filename: file_name
            method: method_type,
        }
    }

    pub fn setup_file(self) {
        FileSetup::new(self.filename).setup_input_file();
    }

    pub fn read_file(&mut self) {
        let file_reader = FileReader::new(self.filename);
        let vector = file_reader.read_file;
        println!("Before operation: {:?}", vector);
        &mut self.vector = Some(vector);
    }

    pub fn match_method(&mut self) {
        let method = match self.method.as_str() {
        "SQUARE" => MathematicalMethod::SQUARE,
        "ROOT" => MathematicalMethod::ROOT,
        _ => {
            eprintln!("function not supported");
            MathematicalMethod::SQUARE
        }
        &mut self.method = method;
    }

    pub fn transform_vectors(&mut self) {
        let mut transformation: Transformation = Transformation::new(&mut self.vector);
        match self.method {
            MathematicalMethod::SQUARE => transformation.square(),
            MathematicalMethod::ROOT => transformation.root(),
            _ => {
                eprintln!("function not supported");
                transformation.root()
            }
        };
        &mut self.vector = transformation.vector;
    }

    pub fn process_input(self) -> <Vec<(i32, i32)> {
        let processor = Processor::new(self.file_name, self.method_type);
        processor.setup_file();
        processor.read_file();
        processor.match_method();
        processor.transform_vectors();
        return processor.vector;
    }
}