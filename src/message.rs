use serde::ser::SerializeMap;

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
pub enum Message {
    System(String),
    User(String),
    Assistant(String),
    ToolCall(Vec<ToolCall>),
    Tool(ToolMessage),
}

impl serde::Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            Message::System(content) => {
                map.serialize_entry("role", "system")?;
                map.serialize_entry("content", content)?;
            },
            Message::User(content) => {
                map.serialize_entry("role", "user")?;
                map.serialize_entry("content", content)?;
            },
            Message::Assistant(content) => {
                map.serialize_entry("role", "assistant")?;
                map.serialize_entry("content", content)?;
            },
            Message::ToolCall(tool_calls) => {
                map.serialize_entry("role", "assistant")?;
                map.serialize_entry("tool_calls", tool_calls)?;
            },
            Message::Tool(tool_message) => {
                map.serialize_entry("role", "tool")?;
                map.serialize_entry("tool_call_id", &tool_message.tool_call_id)?;
                map.serialize_entry("content", &tool_message.content)?;
            },
        }

        map.end()
    }
}

impl <'de> serde::Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MessageVisitor;

        impl<'de> serde::de::Visitor<'de> for MessageVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("chat message")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>, 
            {
                let mut role = None;
                let mut content = None;
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
                    ("system", Some(content), None, None) => Ok(Message::System(content)),
                    ("user", Some(content), None, None) => Ok(Message::User(content)),
                    ("assistant", Some(content), None, None) => Ok(Message::Assistant(content)),
                    ("assistant", None, Some(tool_calls), None) => Ok(Message::ToolCall(tool_calls)),
                    ("tool", Some(content), None, Some(tool_call_id)) => Ok(Message::Tool(ToolMessage { content, tool_call_id })),
                    _ => Err(serde::de::Error::custom("invalid message")),
                }
            }
        }

        deserializer.deserialize_map(MessageVisitor)
    }
}