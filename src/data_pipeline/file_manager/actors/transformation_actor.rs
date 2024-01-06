use tokio::sync::{mpsc, oneshot, mpsc::Sender};

use super::super::super::enums::MathematicalMethod;
use super::super::super::messages::messages::MessageToTransformation;
use super::super::super::messages::messages::MessageToWriter;


/// This struct deals with mathematical transformations
///
/// # Attributes
/// * receiver (mpsc::Receiver<MessageToTransformation>): receiver
/// * sender (Sender<MessageToWriter>): sender
/// * tuple ((i32, i32)): tuple
pub struct TransformationActor {
    pub receiver: mpsc::Receiver<MessageToTransformation>,
    pub sender: mpsc::Sender<MessageToWriter>,
    pub tuple: (i32, i32),
}

/// This is the implementation for the TransformationActor struct.
impl TransformationActor {
    /// This method creates a new TransformationActor struct.
    ///
    /// # Arguments
    /// * receiver (mpsc::Receiver<MessageToTransformation>): receiver
    /// * sender (Sender<MessageToWriter>): sender
    ///
    /// # Returns
    /// (TransformationActor) the newly created struct
    pub fn new(receiver: mpsc::Receiver<MessageToTransformation>, sender: Sender<MessageToWriter>) -> Self {
        return TransformationActor {
            receiver: receiver,
            sender: sender,
            tuple: (0,0)
        }
    }

    /// This method squares all the numbers in a vector.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    fn square(&mut self) {
        self.tuple.1 = self.tuple.1 * self.tuple.1;
    }


    /// This method square roots all the numbers in a vector.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    fn root(&mut self) {
        self.tuple.1 = (self.tuple.1 as f64).sqrt() as i32;
    }

    /// This async method waits until it receives a message and executes when it does
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    pub async fn run(mut self) {
        println!("transformation actor is running");
        while let Some(msg) = self.receiver.recv().await {
            println!("Received message with tuple: {:?}", msg.tuple);
            self.tuple = msg.tuple;
            match msg.mathematical_method {
                MathematicalMethod::SQUARE => self.square(),
                MathematicalMethod::ROOT => self.root(),
                _ => {
                    eprintln!("function not supported");
                }
            };
            println!("Sending transformed message with tuple of {:?} to FileWriterActor...", self.tuple);
            self.send().await;
        }
    }

    /// This async method waits until it receives a message and executes when it does
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    async fn send(&mut self) {
        let (send, recv) = oneshot::channel();
        let message = MessageToWriter { tuple: self.tuple, respond_to: send };
        let _ = self.sender.send(message).await;
        match recv.await {
            Err(e) => println!("{}", e),
            Ok(outcome) => println!("here is the outcome of sending to the writer actor: {}", outcome)
        }
    }
}
