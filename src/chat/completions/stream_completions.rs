use serde_json::json;
use bytes::BytesMut;

use crate::{authen::generate, Error, Result, Sendable};

use super::{request_inner::RequestInner, result::{CompletionChoiceDelta, CompletionResult}, Unpack};

pub struct StreamCompletionsRequest {
    api_key: String,
    inner: RequestInner,
}

impl StreamCompletionsRequest {
    pub(crate) fn new_with(api_key: String, inner: RequestInner) -> Self {
        Self {
            api_key,
            inner,
        }
    }
}

impl Unpack for StreamCompletionsRequest {
    type ExtType = String;

    fn unpack(self) -> (RequestInner, Self::ExtType) {
        (self.inner, self.api_key)
    }

    fn pack(inner: RequestInner, ext: Self::ExtType) -> Self {
        Self { api_key: ext, inner }
    }
}

impl Sendable for StreamCompletionsRequest {
    type Output = CompletionDeltaIter;

    async fn send(self) -> Result<Self::Output> {
        if !self.inner.is_requestable() {
            return Err(Error::MissingParams);
        }

        let token = generate(&self.api_key)?;
        let Ok(mut body) = serde_json::to_value(&self.inner) else {
            return Err(Error::InvalidApiKey);
        };

        body.as_object_mut().unwrap().insert("stream".to_string(), json!(true));

        let response = reqwest::Client::new()
            .post("https://open.bigmodel.cn/api/paas/v4/chat/completions")
            .header("Authorization", format!("Bearer {}", &token))
            .json(&body)
            .send()
            .await?;

        Ok(CompletionDeltaIter{
            response,
            bytes: BytesMut::new(),
            read_eof: false,
        })
    }
}

pub struct CompletionDeltaIter {
    response: reqwest::Response,
    bytes: BytesMut,
    read_eof: bool,
}

impl CompletionDeltaIter {
    pub async fn next(&mut self) -> Result<Option<CompletionResult<CompletionChoiceDelta>>> {
        loop {
            if !self.read_eof {
                self.read_eof = self.read_chunk().await?.is_none();
            }

            let newline_pos = self.bytes.iter().position(|&item| item == b'\n');
            // 如果找到了换行符
            if let Some(pos) = newline_pos {
                // 从缓冲区中提取行的内容
                let line = self.bytes.split_to(pos + 1);
                let line = String::from_utf8_lossy(&line);
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                println!("line:{} end", line);
                
                return to_result(line);
            } else {
                if self.read_eof {
                    let line = self.bytes.split();
                    let line = String::from_utf8_lossy(&line);
                    let line = line.trim();
                    if line.is_empty() {
                        return Err(Error::StreamError);
                    }

                    println!("line:{} end", line);
                    return to_result(line);
                }
            }
        }
    }

    async fn read_chunk(&mut self) -> Result<Option<()>> {
        let Some(chunk) = self.response.chunk().await? else {
            return Ok(None);
        };

        self.bytes.extend_from_slice(&chunk);
        Ok(Some(()))
    }
}

fn to_result(line: &str) -> Result<Option<CompletionResult<CompletionChoiceDelta>>> {
    let Some(line) = line.strip_prefix("data: ") else {
        return Err(Error::StreamError);
    };

    if line == "[DONE]" {
        return Ok(None);
    }

    // 移除已读取的行和换行符
    return Ok(Some(serde_json::from_str::<CompletionResult<CompletionChoiceDelta>>(line)?));
}