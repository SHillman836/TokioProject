use super::super::enums::MathematicalMethod;

/// This struct deals with mathematical transformations
///
/// # Attributes
/// vector (Vec<i32, i32>): vector of bytes
/// method (MathematicalMethod): mathematical method to use
pub struct Transformation {
    pub vector: Vec<(i32, i32)>,
}

/// This is the implementation for the Transformation struct.
impl Transformation {
    /// This method creates a new Transformation struct.
    ///
    /// # Arguments
    /// vector (Vec<i32, i32>): vector of bytes
    /// method (MathematicalMethod): mathematical method to use
    ///
    /// # Returns
    /// (Transformation) the newly created struct
    pub fn new(integers: Vec<(i32, i32)>) -> Self {
        return Transformation {
            vector: integers,
        }
    }

    /// This method squares all the numbers in a vector.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub fn square(&mut self) {
        for tuple in &mut self.vector {
            tuple.1 = tuple.1 * tuple.1;
        }
    }

    /// This method square roots all the numbers in a vector.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub fn root(&mut self) {
        for tuple in &mut self.vector {
            tuple.1 = (tuple.1 as f64).sqrt() as i32;
        }
    }
}
