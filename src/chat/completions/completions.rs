use crate::authen::generate;
use crate::send::Sendable;
use crate::error::{Result, Error};

use super::request_inner::RequestInner;
use super::result::{CompletionChoice, CompletionResult};
use super::stream_completions::StreamCompletionsRequest;
use super::Unpack;

pub struct CompletionsRequestBuilder {
    api_key: String,
    inner: RequestInner,
}

impl CompletionsRequestBuilder {
    pub(crate) fn new(api_key: String) -> Self {
        Self {
            api_key,
            inner: RequestInner::new(),
        }
    }

    pub fn stream(self) -> StreamCompletionsRequest {
        StreamCompletionsRequest::new_with(self.api_key, self.inner)
    }
}

impl Unpack for CompletionsRequestBuilder {
    type ExtType = String;

    fn unpack(self) -> (RequestInner, Self::ExtType) {
        (self.inner, self.api_key)
    }

    fn pack(inner: RequestInner, ext: Self::ExtType) -> Self {
        Self { api_key: ext, inner }
    }
}

impl Sendable for CompletionsRequestBuilder {
    type Output = CompletionResult<CompletionChoice>;

    async fn send(self) -> Result<Self::Output> {
        if !self.inner.is_requestable() {
            return Err(Error::MissingParams);
        }

        let token = generate(&self.api_key)?;

        let ret = reqwest::Client::new()
            .post("https://open.bigmodel.cn/api/paas/v4/chat/completions")
            .header("Authorization", format!("Bearer {}", &token))
            .json(&self.inner)
            .send()
            .await?
            .json::<CompletionResult<CompletionChoice>>()
            .await?;
        Ok(ret)
    }
}
