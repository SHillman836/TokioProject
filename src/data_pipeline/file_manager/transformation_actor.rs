use super::super::enums::MathematicalMethod;
use super::super::messages::messages::MessageToWriter


/// This struct deals with mathematical transformations
///
/// # Attributes
/// vector (Vec<i32, i32>): vector of bytes
/// method (MathematicalMethod): mathematical method to use
pub struct Transformation {
    pub vector: Vec<(i32, i32)>,
    pub sender: Sender<Message>,
    pub receiver: mpsc::Receiver<Message>
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

    /// This async method waits until it receives a message and executes when it does
    ///
    /// # Arguments
    /// * message (Message>): the message to write the the file
    ///
    /// # Returns
    /// None
    async fn run(mut self) {
        println!("transformation actor is running");
        while let Some(msg) = self.receiver.recv().await {
            match msg.mathematical_method {
                MathematicalMethod::SQUARE => msg.vector.square(),
                MathematicalMethod::ROOT => msg.vector.root(),
                _ => {
                    eprintln!("function not supported");
                    transformation.root()
                }
            };
            self.send(msg.vector)
        }
    }

    /// This async method waits until it receives a message and executes when it does
    ///
    /// # Arguments
    /// * vector (Vec<i32, i32>): the message to write the the file
    ///
    /// # Returns
    /// None
    async fn send(self, vector: Vec<i32, i32>) {
        let (send, recv) = oneshot::channel();
        let message = MessageToWriter { vector: vector, respond_to: send };
        let _ = self.sender.send(message).await;
        match recv.await {
            Err(e) => println!("{}", e),
            Ok(outcome) => println!("here is the outcome of sending to the writer actor: {}", outcome)
        }
    }
}
