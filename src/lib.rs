pub mod openglm;
pub mod chat;
pub mod send;
pub mod error;
pub mod authen;

pub mod prelude {
    pub use super::openglm::OpenGLM;
    pub use super::error::{Error, Result};
    pub use super::send::Sendable;
    pub use super::chat::{chat::*, tools::*, message::*, completions::{result::*, request_inner::{Unpack, RequestBuild}}};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[tokio::test]
    async fn test_chat() {
        let client = OpenGLM::new("1111111111111111111111.xxxxxxx".to_string());
        let result = client.chat().completions().create()
            .with_model("glm-4".to_string())
            .add_message(ChatMessage::System("你是一个聪明且富有创造力的小说作家".to_string()))
            .add_message(ChatMessage::User("请你作为童话故事大王，写一篇短篇童话故事，故事的主题是要永远保持一颗善良的心，要能够激发儿童的学习兴趣和想象力，同时也能够帮助儿童更好地理解和接受故事中所蕴含的道理和价值观。".to_string()))
            .send().await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_stream_chat() {
        let client = OpenGLM::new("1111111111111111111111.xxxxxxx".to_string());
        let mut result = client.chat().completions().create()
            .with_model("glm-4".to_string())
            .add_message(ChatMessage::System("你是一个聪明且富有创造力的小说作家".to_string()))
            .add_message(ChatMessage::User("请你作为童话故事大王，写一篇短篇童话故事，故事的主题是要永远保持一颗善良的心，要能够激发儿童的学习兴趣和想象力，同时也能够帮助儿童更好地理解和接受故事中所蕴含的道理和价值观。".to_string()))
            .stream()
            .send().await.unwrap();

        while let Some(delta) = result.next().await.unwrap() {
            println!("{:?}", delta);
        }
    }
}
