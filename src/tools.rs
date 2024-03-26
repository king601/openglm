use serde::ser::SerializeMap;

#[derive(serde::Serialize, Default)]
pub struct FunctionTool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(serde::Serialize, Default)]
pub struct Retrieval {
    pub knowledge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,
}

#[derive(serde::Serialize, Default)]
pub struct WebSearch {
    pub search_query: String,
    pub enable: bool,
}

pub enum Tool {
    Function(FunctionTool),
    Retrieval(Retrieval),
    WebSearch(WebSearch),
}

impl serde::Serialize for Tool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        match self {
            Tool::Function(function) => {
                map.serialize_entry("type", "function")?;
                map.serialize_entry("function", function)?;
            },
            Tool::Retrieval(retrieval) => {
                map.serialize_entry("type", "retrieval")?;
                map.serialize_entry("retrieval", retrieval)?;
            },
            Tool::WebSearch(web_search) => {
                map.serialize_entry("type", "web_search")?;
                map.serialize_entry("web_search", web_search)?;
            },
        }

        map.end()
    }
}