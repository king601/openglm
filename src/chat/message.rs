use serde::ser::SerializeMap;

use crate::error::Error;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Function {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub function: Function,
}

#[derive(Debug)]
pub struct ToolMessage {
    pub content: String,
    pub tool_call_id: String,
}

#[derive(Debug)]
pub enum ImageMessage {
    Text(String),
    ImageUrl(String),
}

// 使用一个辅助结构体来正确地序列化ImageUrl
#[derive(serde::Serialize)]
struct ImageUrlWrapper<'a> {
    url: &'a str,
}

impl serde::Serialize for ImageMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            ImageMessage::Text(text) => {
                map.serialize_entry("type", "text")?;
                map.serialize_entry("text", text)?;
            },
            ImageMessage::ImageUrl(url) => {
                map.serialize_entry("type", "image_url")?;
                map.serialize_entry("image_url", &ImageUrlWrapper { url: url })?;
            },
        }
        map.end()
    }

}

#[derive(Debug)]
pub enum ChatMessage {
    System(String),
    User(String),
    Image(Vec<ImageMessage>),
    Assistant(String),
    ToolCall(Vec<ToolCall>),
    Tool(ToolMessage),
}

impl serde::Serialize for ChatMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            ChatMessage::System(content) => {
                map.serialize_entry("role", "system")?;
                map.serialize_entry("content", content)?;
            },
            ChatMessage::User(content) => {
                map.serialize_entry("role", "user")?;
                map.serialize_entry("content", content)?;
            },
            ChatMessage::Image(images) => {
                map.serialize_entry("role", "user")?;
                map.serialize_entry("content", images)?;
            },
            ChatMessage::Assistant(content) => {
                map.serialize_entry("role", "assistant")?;
                map.serialize_entry("content", content)?;
            },
            ChatMessage::ToolCall(tool_calls) => {
                map.serialize_entry("role", "assistant")?;
                map.serialize_entry("tool_calls", tool_calls)?;
            },
            ChatMessage::Tool(tool_message) => {
                map.serialize_entry("role", "tool")?;
                map.serialize_entry("tool_call_id", &tool_message.tool_call_id)?;
                map.serialize_entry("content", &tool_message.content)?;
            },
        }

        map.end()
    }
}

impl <'de> serde::Deserialize<'de> for ChatMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MessageVisitor;

        impl<'de> serde::de::Visitor<'de> for MessageVisitor {
            type Value = ChatMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("chat message")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, 
            {
                let mut role = None;
                let mut content: Option<serde_json::Value> = None;
                let mut tool_calls: Option<Vec<ToolCall>> = None;
                let mut tool_call_id: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "role" => role = map.next_value()?,
                        "content" => content = map.next_value()?,
                        "tool_calls" => tool_calls = map.next_value()?,
                        "tool_call_id" => tool_call_id = map.next_value()?,
                        _ => return Err(serde::de::Error::unknown_field(key, &["role", "content", "tool_calls", "tool_call_id"])),
                    }
                }

                let role: String = role.ok_or_else(|| serde::de::Error::missing_field("role"))?;

                match (role.as_str(), content, tool_calls, tool_call_id) {
                    ("system", Some(serde_json::Value::String(content)), None, None) => Ok(ChatMessage::System(content)),
                    ("user", Some(serde_json::Value::String(content)), None, None) => Ok(ChatMessage::User(content)),
                    ("user", Some(serde_json::Value::Array(content)), None, None) => {
                        let mut images = Vec::new();
                        for image in content {
                            let image = image.as_object().ok_or(serde::de::Error::custom("invalid image format"))?;
                            match image.get("type") {
                                Some(serde_json::Value::String(ty)) => {
                                    match ty.as_str() {
                                        "text" => {
                                            let content = image.get("text").ok_or(serde::de::Error::custom("missing text field"))?;
                                            let content = content.as_str().ok_or(serde::de::Error::custom("invalid text field"))?;
                                            images.push(ImageMessage::Text(content.to_string()));
                                        },
                                        "image_url" => {
                                            let image_url = image.get("image_url").ok_or(serde::de::Error::custom("missing image_url field"))?;
                                            let image_url = image_url.as_object().ok_or(serde::de::Error::custom("invalid image_url field"))?;
                                            let url = image_url.get("url").ok_or(serde::de::Error::custom("missing url field"))?;
                                            let url = url.as_str().ok_or(serde::de::Error::custom("invalid url field"))?;
                                            images.push(ImageMessage::ImageUrl(url.to_string()));
                                        }
                                        _ => return Err(serde::de::Error::custom("invalid image type")),
                                    }
                                },
                                _ => return Err(serde::de::Error::custom("invalid image type")),
                            }
                        }
                        Ok(ChatMessage::Image(images))
                    },
                    ("assistant", Some(serde_json::Value::String(content)), None, None) => Ok(ChatMessage::Assistant(content)),
                    ("assistant", None, Some(tool_calls), None) => Ok(ChatMessage::ToolCall(tool_calls)),
                    ("tool", Some(serde_json::Value::String(content)), None, Some(tool_call_id)) => Ok(ChatMessage::Tool(ToolMessage { content, tool_call_id })),
                    _ => Err(serde::de::Error::custom("invalid message")),
                }
            }
        }

        deserializer.deserialize_map(MessageVisitor)
    }
}

#[derive(Debug)]
pub enum AssistantMessageDelta {
    Content(String),
    ToolCall(Vec<ToolCall>),
}

impl <'de> serde::Deserialize<'de> for AssistantMessageDelta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        struct DeltaVisitor;

        impl<'de> serde::de::Visitor<'de> for DeltaVisitor {
            type Value = AssistantMessageDelta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("assistant message delta")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, 
            {
                let mut role = None;
                let mut content = None;
                let mut tool_calls: Option<Vec<ToolCall>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "role" => role = map.next_value()?,
                        "content" => content = map.next_value()?,
                        "tool_calls" => tool_calls = map.next_value()?,
                        _ => return Err(serde::de::Error::unknown_field(key, &["role", "content", "tool_calls"])),
                    }
                }

                let role: String = role.ok_or_else(|| serde::de::Error::missing_field("role"))?;

                match (role.as_str(), content, tool_calls) {
                    ("assistant", Some(content), None) => Ok(AssistantMessageDelta::Content(content)),
                    ("assistant", None, Some(tool_calls)) => Ok(AssistantMessageDelta::ToolCall(tool_calls)),
                    _ => Err(serde::de::Error::custom("invalid message")),
                }
            }
        }

        deserializer.deserialize_map(DeltaVisitor)
    }
}

impl TryFrom<Vec<AssistantMessageDelta>> for ChatMessage {
    type Error = Error;
    
    fn try_from(value: Vec<AssistantMessageDelta>) -> Result<Self, Self::Error> {
        let mut message = None;
        for delta in value {
            match delta {
                AssistantMessageDelta::Content(income) => {
                    match message {
                        Some(ChatMessage::Assistant(ref mut content)) => {
                            content.push_str(&income);
                        },
                        None => {
                            message = Some(ChatMessage::Assistant(income));
                        },
                        _ => return Err(Error::Conflict),
                    }
                },
                AssistantMessageDelta::ToolCall(mut income) => {
                    match message {
                        Some(ChatMessage::ToolCall(ref mut tool_calls)) => {
                            tool_calls.append(&mut income);
                        },
                        None => {
                            message = Some(ChatMessage::ToolCall(income));
                        },
                        _ => return Err(Error::Conflict),
                    }
                }
            }
        }

        message.ok_or(Error::EmptyDeltaList)
    }
}