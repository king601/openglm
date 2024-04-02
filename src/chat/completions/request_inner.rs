use crate::chat::{message::ChatMessage, tools::*};

#[derive(serde::Serialize)]
pub struct RequestInner {
    model: Option<String>,
    messages: Option<Vec<ChatMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    do_sample: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
}

impl RequestInner {
    pub(crate) fn new() -> Self {
        Self {
            model: None,
            messages: None,
            request_id: None,
            do_sample: None,
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop: None,
            tools: None,
            tool_choice: None,
        }
    }

    pub(crate) fn is_requestable(&self) -> bool {
        self.model.is_some() && self.messages.is_some() && !self.messages.as_ref().unwrap().is_empty()
    }

    pub(crate) fn with_model(self, model: String) -> Self {
        Self {
            model: Some(model),
            ..self
        }
    }

    pub(crate) fn with_messages(self, messages: Vec<ChatMessage>) -> Self {
        Self {
            messages: Some(messages),
            ..self
        }
    }

    pub(crate) fn add_message(self, message: ChatMessage) -> Self {
        let mut messages = self.messages.unwrap_or_default();
        messages.push(message);

        Self { 
            messages: Some(messages), 
            ..self 
        }
    }

    pub(crate) fn with_request_id(self, request_id: String) -> Self {
        Self {
            request_id: Some(request_id),
            ..self
        }
    }

    pub(crate) fn with_do_sample(self, do_sample: bool) -> Self {
        Self {
            do_sample: Some(do_sample),
            ..self
        }
    }

    pub(crate) fn with_temperature(self, temperature: f32) -> Self {
        Self {
            temperature: Some(temperature),
            ..self
        }
    }

    pub(crate) fn with_top_p(self, top_p: f32) -> Self {
        Self {
            top_p: Some(top_p),
            ..self
        }
    }

    pub(crate) fn with_max_tokens(self, max_tokens: i32) -> Self {
        Self {
            max_tokens: Some(max_tokens),
            ..self
        }
    }

    pub(crate) fn with_stop(self, stop: Vec<String>) -> Self {
        Self {
            stop: Some(stop),
            ..self
        }
    }

    pub(crate) fn bind_function(self, function: FunctionTool) -> Self {
        let mut tools = self.tools.unwrap_or_default();
        tools.push(Tool::Function(function));

        Self { 
            tools: Some(tools), 
            ..self 
        }
    }

    pub(crate) fn bind_retrieval(self, retrieval: Retrieval) -> Self {
        let mut tools = self.tools.unwrap_or_default();
        tools.push(Tool::Retrieval(retrieval));

        Self { 
            tools: Some(tools), 
            ..self 
        }
    }

    pub(crate) fn bind_web_search(self, web_search: WebSearch) -> Self {
        let mut tools = self.tools.unwrap_or_default();
        tools.push(Tool::WebSearch(web_search));

        Self { 
            tools: Some(tools), 
            ..self 
        }
    }

    pub(crate) fn with_tool_choice(self, tool_choice: String) -> Self {
        Self {
            tool_choice: Some(tool_choice),
            ..self
        }
    }
}

pub trait Unpack {
    type ExtType;

    fn unpack(self) -> (RequestInner, Self::ExtType);
    fn pack(inner: RequestInner, ext: Self::ExtType) -> Self;
}

pub trait RequestBuild {
    fn with_model(self, model: String) -> Self;
    fn with_messages(self, messages: Vec<ChatMessage>) -> Self;
    fn add_message(self, message: ChatMessage) -> Self;
    fn with_request_id(self, request_id: String) -> Self;
    fn with_do_sample(self, do_sample: bool) -> Self;
    fn with_temperature(self, temperature: f32) -> Self;
    fn with_top_p(self, top_p: f32) -> Self;
    fn with_max_tokens(self, max_tokens: i32) -> Self;
    fn with_stop(self, stop: Vec<String>) -> Self;
    fn bind_function(self, function: FunctionTool) -> Self;
    fn bind_retrieval(self, retrieval: Retrieval) -> Self;
    fn bind_web_search(self, web_search: WebSearch) -> Self;
    fn with_tool_choice(self, tool_choice: String) -> Self;
}

impl <T: Unpack> RequestBuild for T {
    fn with_model(self, model: String) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_model(model), ext)
    }

    fn with_messages(self, messages: Vec<ChatMessage>) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_messages(messages), ext)
    }

    fn add_message(self, message: ChatMessage) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.add_message(message), ext)
    }

    fn with_request_id(self, request_id: String) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_request_id(request_id), ext)
    }

    fn with_do_sample(self, do_sample: bool) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_do_sample(do_sample), ext)
    }

    fn with_temperature(self, temperature: f32) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_temperature(temperature), ext)
    }

    fn with_top_p(self, top_p: f32) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_top_p(top_p), ext)
    }

    fn with_max_tokens(self, max_tokens: i32) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_max_tokens(max_tokens), ext)
    }

    fn with_stop(self, stop: Vec<String>) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_stop(stop), ext)
    }

    fn bind_function(self, function: FunctionTool) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.bind_function(function), ext)
    }

    fn bind_retrieval(self, retrieval: Retrieval) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.bind_retrieval(retrieval), ext)
    }

    fn bind_web_search(self, web_search: WebSearch) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.bind_web_search(web_search), ext)
    }

    fn with_tool_choice(self, tool_choice: String) -> Self {
        let (inner, ext) = self.unpack();
        Self::pack(inner.with_tool_choice(tool_choice), ext)
    }
} 