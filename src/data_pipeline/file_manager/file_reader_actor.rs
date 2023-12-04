use tokio::sync::{mpsc, oneshot, mpsc::Sender};

use std::fs::File;

use super::message::Message

/// This struct defines a file reader actor.
///
/// # Attributes
/// * filename (&str): name of the file to read
/// * sender (Sender<Message>): sender
pub struct FileReaderActor {
    pub vector: <Vec<(i32, i32)>,
    pub sender: Sender<Message>
}

/// This is the implementation for the FileReaderActor struct.
impl FileReaderActor {
    /// This function creates a new FileReaderActor struct.
    ///
    /// # Arguments
    /// * filename (&str): the name of the file
    /// * sender (Sender<Message>): sender
    ///
    /// # Returns
    /// (FileReaderActor) the newly created struct
    fn new(vector: <Vec<(i32, i32)>, sender: Sender<Message>) -> Self {
        return FileReaderActor {
            vector,
            sender
        }
    }

    /// This async function takes in a vector and sends it in a message.
    ///
    /// # Arguments
    /// * vector (Vec<(i32,i32)>): the name of the file
    ///
    /// # Returns
    /// None
    async fn send(self) {
        let (send, recv) = oneshot::channel();
        let message = Message { vector: self.vector,
                                respond_to: send };
        let _ = self.sender.send(message).await;
        match recv.await {
            Err(e) => println!("{}", e),
            Ok(outcome) => println!("here is the outcome: {}", outcome)
        }
    }
}

}