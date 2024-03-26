use crate::authen::generate;
use crate::message::Message;
use crate::Sendable;
use crate::error::{Result, Error};

use super::request_inner::RequestInner;
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

#[derive(serde::Deserialize, Debug)]
pub struct CompletionResult {
    pub id: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(serde::Deserialize, Debug)]
pub struct CompletionChoice {
    pub index: i32,
    pub finish_reason: String,
    pub message: Message,
}

#[derive(serde::Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

impl Sendable for CompletionsRequestBuilder {
    type Output = CompletionResult;

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
            .json::<CompletionResult>()
            .await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::authen::generate;

    use super::CompletionResult;

    #[tokio::test]
    async fn test_chat() {
        let client = OpenGLM::new("b0bd15d8b10aa938a9bb52faee28772f.iAPGMaNCsZzruiyK".to_string());
        let builder = client.chat().completions().create()
            .with_model("glm-4".to_string())
            .add_message(Message::System("你是一个聪明且富有创造力的小说作家".to_string()))
            .add_message(Message::User("请你作为童话故事大王，写一篇短篇童话故事，故事的主题是要永远保持一颗善良的心，要能够激发儿童的学习兴趣和想象力，同时也能够帮助儿童更好地理解和接受故事中所蕴含的道理和价值观。".to_string()));

        let token = generate(&builder.api_key).unwrap();

        let ret = reqwest::Client::new()
            .post("https://open.bigmodel.cn/api/paas/v4/chat/completions")
            .header("Authorization", format!("Bearer {}", &token))
            .json(&builder.inner)
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();
        println!("{}", ret);
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"choices":[{"finish_reason":"stop","index":0,"message":{"content":"《魔法森林的奇妙之旅》\n\n从前，有个小女孩叫莉莉，她有一颗善良的心，热爱帮助他人。一天，她在图书馆无意间发现了一本古老的童话书，书中描绘了一个神秘的魔法森林。\n\n好奇心驱使莉莉翻开这本书，突然，一道耀眼的光芒从书中迸发出来，将她带到了一个神奇的世界——魔法森林。\n\n在这片神奇的森林里，树木会说话，小动物们会唱歌跳舞。莉莉遇到了一位可爱的小精灵，他叫小智。小智告诉莉莉，魔法森林正遭受一场灾难，一个邪恶的巫婆把森林的魔法源泉给封印了，如果不解开这个封印，森林里的生物都将失去魔法，陷入无尽的黑暗。\n\n莉莉决定帮助小智解救魔法森林。他们历经千辛万苦，翻山越岭，遇到了各种各样神奇的生物。在每个困难的时刻，莉莉都凭借着自己的善良和勇敢，克服了困难。\n\n最后，他们来到了邪恶巫婆的城堡。巫婆嘲笑莉莉，说：“你这个愚蠢的人类，也妄想解开我的封印？”莉莉毫不畏惧，坚定地说：“我有一颗善良的心，我相信善良的力量可以战胜一切邪恶！”\n\n巫婆愤怒地施展出强大的魔法，企图击败莉莉。然而，莉莉心中的善良之光越来越耀眼，她用善良的力量化解了巫婆的攻击。最后，善良之光照射在封印上，解开了魔法源泉的封印。\n\n魔法森林恢复了往日的生机，所有生物都感谢莉莉的帮助。小智含泪送别莉莉，说：“谢谢你，莉莉，是你让我们重新拥有了魔法。请你永远保持这颗善良的心，它会给你带来无尽的快乐。”\n\n突然，光芒再次闪现，莉莉回到了现实世界。她紧紧握住手中的童话书，心中充满了信念：她要永远保持善良的心，用善良的力量去帮助更多的人。\n\n这个故事让小朋友们明白，善良是一种强大的力量，它能帮助我们战胜困难，驱散黑暗。同时，故事中充满奇幻色彩的魔法森林和生动有趣的角色，也激发了孩子们的学习兴趣和想象力。希望大家都能像莉莉一样，永远保持一颗善良的心。","role":"assistant"}}],"created":1711434535,"id":"8512982659489624459","model":"glm-4","request_id":"8512982659489624459","usage":{"completion_tokens":445,"prompt_tokens":61,"total_tokens":506}}"#;
        let _: CompletionResult = serde_json::from_str(json).unwrap();
    }
}