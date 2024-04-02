use crate::chat::completions::completions::CompletionsRequestBuilder;

pub struct Chat {
    api_key: String,
}

impl Chat {
    pub(crate) fn new(api_key: String) -> Self {
        Self {
            api_key,
        }
    }

    pub fn completions(self) -> Completions {
        Completions::new(self.api_key)
    }
}

pub struct Completions {
    api_key: String,
}

impl Completions {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
        }
    }

    pub fn create(self) -> CompletionsRequestBuilder {
        CompletionsRequestBuilder::new(self.api_key.clone())
    }
}
