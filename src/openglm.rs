use std::{cell::RefCell, rc::Rc};

use crate::chat::chat::Chat;

pub struct OpenGLM {
    api_key: Rc<RefCell<String>>,
}

impl OpenGLM {
    pub fn new(api_key: String) -> Self {
        let api_key = Rc::new(RefCell::new(api_key));
        Self {
            api_key,
        }
    }

    pub fn chat(&self) -> Chat {
        Chat::new(self.api_key.clone())
    }
}
