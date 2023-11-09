use super::enums::MathematicalMethod;

pub struct Transformation {
    pub vector: Vec<(i32, i32)>,
    pub method: MathematicalMethod
}

impl Transformation {
    pub fn new(integers: Vec<(i32, i32)>, method_type: MathematicalMethod) -> Self {
        return Transformation {
            vector: integers,
            method: method_type
        }
    }

    pub fn square(&mut self) {
        for tuple in &mut self.vector {
            tuple.1 = tuple.1 * tuple.1;
        }
    }

    pub fn root(&mut self) {
        for tuple in &mut self.vector {
            tuple.1 = (tuple.1 as f64).sqrt() as i32;
        }
    }
}
