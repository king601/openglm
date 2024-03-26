mod openglm;
mod chat;
mod message;
mod send;
mod completions;
mod tools;
mod error;
mod authen;

pub use openglm::*;
pub use send::*;
pub use tools::*;
pub use error::*;
pub use message::*;
pub use completions::{Unpack, RequestBuild};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat() {
        let client = OpenGLM::new("b0bd15d8b10aa938a9bb52faee28772f.iAPGMaNCsZzruiyK".to_string());
        let result = client.chat().completions().create()
            .with_model("glm-4".to_string())
            .add_message(Message::System("你是一个聪明且富有创造力的小说作家".to_string()))
            .add_message(Message::User("请你作为童话故事大王，写一篇短篇童话故事，故事的主题是要永远保持一颗善良的心，要能够激发儿童的学习兴趣和想象力，同时也能够帮助儿童更好地理解和接受故事中所蕴含的道理和价值观。".to_string()))
            .send().await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_stream_chat() {
        let client = OpenGLM::new("b0bd15d8b10aa938a9bb52faee28772f.iAPGMaNCsZzruiyK".to_string());
        let mut result = client.chat().completions().create()
            .with_model("glm-4".to_string())
            .add_message(Message::System("你是一个聪明且富有创造力的小说作家".to_string()))
            .add_message(Message::User("请你作为童话故事大王，写一篇短篇童话故事，故事的主题是要永远保持一颗善良的心，要能够激发儿童的学习兴趣和想象力，同时也能够帮助儿童更好地理解和接受故事中所蕴含的道理和价值观。".to_string()))
            .stream()
            .send().await.unwrap();

        while let Some(delta) = result.next().await.unwrap() {
            println!("{:?}", delta);
        }
    }
}