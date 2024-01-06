/// Represents different types of mathematical methods.
///
/// # Variants
/// * `SQUARE` - Represents the operation of squaring a number.
/// * `ROOT` - Represents the operation of finding the square root of a number.
#[derive(Clone)]
#[derive(Debug)]
pub enum MathematicalMethod {
    SQUARE,
    ROOT,
}

impl MathematicalMethod {
    /// Converts the mathematical method to its corresponding string representation.
    ///
    /// This method returns a `String` that represents the current state of the mathematical operation.
    ///
    /// # Returns
    /// (String) that is:
    /// - `"DONE"` when the method is `SQUARE`.
    /// - `"PENDING"` when the method is `ROOT`.
    pub fn stringify(&self) -> String {
        match &self {
            &Self::SQUARE => "DONE".to_string(),
            &Self::ROOT => "PENDING".to_string(),
        }
    }
}
