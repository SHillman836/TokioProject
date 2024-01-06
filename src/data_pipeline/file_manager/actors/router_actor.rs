use tokio::sync::mpsc;
use super::super::super::messages::messages::MessageToTransformation;

pub struct RouterActor {
    pub senders: Vec<mpsc::Sender<MessageToTransformation>>,
    pub current_index: usize,
}

impl RouterActor {
    pub fn new(senders: Vec<mpsc::Sender<MessageToTransformation>>) -> Self {
        RouterActor { senders, current_index: 0 }
    }

    pub async fn route(&mut self, message: MessageToTransformation) {
        if let Some(sender) = self.senders.get(self.current_index) {
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
