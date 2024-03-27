use crate::chat::message::{Message, AssistantMessageDelta};

#[derive(serde::Deserialize, Debug)]
pub struct CompletionResult<T> {
    pub id: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<T>,
    pub usage: Option<Usage>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct CompletionChoice {
    pub index: i32,
    pub finish_reason: String,
    pub message: Message,
}

#[derive(Debug, serde::Deserialize)]
pub struct CompletionChoiceDelta {
    pub index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    pub delta: AssistantMessageDelta,
}