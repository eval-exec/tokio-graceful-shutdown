use tokio::sync::watch;

pub struct Event {
    receiver: watch::Receiver<bool>,
}

impl Event {
    pub fn create() -> (Self, impl FnOnce()) {
        let (sender, receiver) = watch::channel(false);
        (Self { receiver }, move || {
            sender.send_replace(true);
        })
    }

    pub fn get(&self) -> bool {
        *self.receiver.borrow()
    }

    pub async fn wait(&self) {
        self.receiver.clone().changed().await.unwrap();
    }
}
