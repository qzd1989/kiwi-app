use super::{Level, Message};
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct Log {
    dir: PathBuf,
}

impl Log {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn info(&self, message: impl Into<String>) {
        let msg = message.into();
        println!("{:#?}", &msg);
        let _ = Message::new(Level::Info, msg).save(&self.dir);
    }

    pub fn warn(&self, message: impl Into<String>) {
        let msg = message.into();
        println!("{:#?}", &msg);
        let _ = Message::new(Level::Warn, msg).save(&self.dir);
    }

    pub fn error(&self, message: impl Into<String>) {
        let msg = message.into();
        println!("{:#?}", &msg);
        let _ = Message::new(Level::Error, msg).save(&self.dir);
    }
}
