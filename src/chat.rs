use std::{cell::RefCell, rc::Rc};

use crate::completions::completions::CompletionsRequestBuilder;

pub struct Chat {
    api_key: Rc<RefCell<String>>,
}

impl Chat {
    pub(crate) fn new(api_key: Rc<RefCell<String>>) -> Self {
        Self {
            api_key,
        }
    }

    pub fn completions(self) -> Completions {
        Completions::new(self.api_key)
    }
}

pub struct Completions {
    api_key: Rc<RefCell<String>>,
}

impl Completions {
    pub fn new(api_key: Rc<RefCell<String>>) -> Self {
        Self {
            api_key,
        }
    }

    pub fn create(self) -> CompletionsRequestBuilder {
        CompletionsRequestBuilder::new(self.api_key.as_ref().borrow().clone())
    }
}
