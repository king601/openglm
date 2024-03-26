use super::{request_inner::RequestInner, Unpack};

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

