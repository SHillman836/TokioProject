use tokio::sync::{mpsc, oneshot, mpsc::Sender};
use super::super::enums::MathematicalMethod;

/// This struct defines a message
///
/// # Attributes
/// * vector (Vec<(i32, i32)>): a vector of integers
/// * mathematical_method (<MathematicalMethod>): the mathematical processing method
/// * respond_to (oneshot::Sender<u32>): the message sender
#[derive(Debug)]
pub struct MessageToTransformation {
    pub vector: Vec<(i32, i32)>,
    pub mathematical_method: MathematicalMethod,
    pub respond_to: oneshot::Sender<u32>
}

/// This struct defines a message
///
/// # Attributes
/// * vector (Vec<(i32, i32)>): a vector of integers
/// * respond_to (oneshot::Sender<u32>): the message sender
#[derive(Debug)]
pub struct MessageToWriter {
    pub vector: Vec<(i32, i32)>,
    pub respond_to: oneshot::Sender<u32>
}