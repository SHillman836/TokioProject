#[derive(Educe)]
#[educe(Clone)]
pub enum MathematicalMethod {
    SQUARE,
    ROOT
}

impl MathematicalMethod {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::SQUARE => {"DONE".to_string()}
            &Self::ROOT => {"PENDING".to_string()}
        }
    }
}