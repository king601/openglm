use crate::chat::chat::Chat;

pub struct OpenGLM {
    api_key: String,
}

impl OpenGLM {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
        }
    }

    pub fn chat(&self) -> Chat {
        Chat::new(self.api_key.clone())
    }
}
