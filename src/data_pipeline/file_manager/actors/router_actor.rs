use tokio::sync::mpsc;
use super::super::super::messages::messages::MessageToTransformation;

/// This defines creates a RouterActor struct.
///
/// # Attributes
/// * senders (Vec<mpsc::Sender<MessageToTransformation>>): vector of senders to choose from
/// * current_index (usize): the current index of the sender the router is routing to
pub struct RouterActor {
    pub senders: Vec<mpsc::Sender<MessageToTransformation>>,
    pub current_index: usize,
}

/// This is the implementation for the RouterActor struct.
impl RouterActor {
    /// This method creates a new RouterActor struct.
    ///
    /// # Arguments
    /// * senders (Vec<mpsc::Sender<MessageToTransformation>>): vector of senders to choose from
    ///
    /// # Returns
    /// (RouterActor) The newly created struct
    pub fn new(senders: Vec<mpsc::Sender<MessageToTransformation>>) -> Self {
        RouterActor { senders, current_index: 0 }
    }

    /// This method routes the message to the senders.
    ///
    /// # Arguments
    /// * message (MessageToTransformation): message to send to the transformation actor
    ///
    /// # Returns
    /// (RouterActor) The newly created struct
    pub async fn route(&mut self, message: MessageToTransformation) {
        println!("Router progress: Routing to index {}", self.current_index);
        if let Some(sender) = self.senders.get(self.current_index) {
            println!("Sending message to TransformationActor at index {}", self.current_index); // New line added
            if sender.send(message).await.is_err() {
                println!("Failed to send message to TransformationActor");
            }
        } else {
            eprintln!("No available TransformationActor");
        }

        // Move to the next sender in a round-robin fashion
        self.current_index = (self.current_index + 1) % self.senders.len();
    }
}
