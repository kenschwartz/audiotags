use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct MbValue {
    pub value: RwLock<Arc<str>>,
}


impl Default for MbValue {
    fn default() -> Self {
        Self {
            value: RwLock::new(Arc::from(String::new())),
        }
    }
}

impl MbValue {
    pub fn value(&self) -> Arc<str> {
        let read_lock = self
            .value
            .read()
            .expect("MbValue: error getting read");
        return Arc::clone(&*read_lock);
    }
    pub fn set_value(&self, val: &str) {
        let mut write_lock = self
            .value
            .write()
            .expect("MbValue: error getting write");
        *write_lock = Arc::from(val);
    }
}

impl Clone for MbValue {
    fn clone(&self) -> Self {
        MbValue {
            value: RwLock::new(self.value()),
        }
    }
}