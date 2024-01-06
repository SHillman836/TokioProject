use tokio::sync::{mpsc, oneshot, mpsc::Sender};
use super::super::enums::MathematicalMethod;

/// This struct defines a message
///
/// # Attributes
/// * tuple ((i32, i32)): a tuple of integers
/// * mathematical_method (<MathematicalMethod>): the mathematical processing method
#[derive(Debug)]
pub struct MessageToTransformation {
    pub tuple: (i32, i32),
    pub mathematical_method: MathematicalMethod,
}

/// This struct defines a message
///
/// # Attributes
/// * tuple ((i32, i32)): a tuple of integers
/// * respond_to (oneshot::Sender<u32>): the message sender
#[derive(Debug)]
pub struct MessageToWriter {
    pub tuple: (i32, i32),
    pub respond_to: oneshot::Sender<u32>
}