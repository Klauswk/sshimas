use std::result::{Result};
use common::ConnectionData;

pub trait Add {
    fn add(&self, connection: &ConnectionData) -> Result<&str, &str>;

    fn help(&self) -> Option<&str> {
        None
    }
}

pub trait Connect {
    fn connect(&self, connection: &ConnectionData);

    fn help(&self) -> Option<&str> {
        None
    }
}

pub trait Remove {
    fn remove(&self, connection: &ConnectionData) -> Result<&str, &str>;

    fn help(&self) -> Option<&str> {
        None
    }
}

pub trait Get {
    fn get(&self, id: &str) -> Result<ConnectionData, &str>;

    fn help(&self) -> Option<&str> {
        None
    }
}

pub trait List {
    fn list(&self) -> Result<Vec<ConnectionData>, &str>;

    fn help(&self) -> Option<&str> {
        None
    }
}

pub trait History {
    fn history(&self);
	
	fn append(&self, connection: &ConnectionData);
	
    fn help(&self) -> Option<&str> {
        None
    }
}