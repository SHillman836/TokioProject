use tokio::sync::{mpsc, oneshot, mpsc::Sender};

/// This struct defines a message
///
/// # Attributes
/// * vector (Vec<(i32, i32)>): a vector of integers
/// * respond_to (oneshot::Sender<u32>): the message sender
#[derive(Debug)]
pub struct Message {
    pub vector: Vec<(i32, i32)>,
    pub respond_to: oneshot::Sender<u32>
}