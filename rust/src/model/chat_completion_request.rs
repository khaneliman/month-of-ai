use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

pub struct MessageBuilder {
    role: Option<String>,
    content: Option<String>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
            role: None,
            content: None,
        }
    }

    pub fn role(mut self, role: String) -> Self {
        self.role = Some(role);
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn build(self) -> Message {
        Message {
            role: self.role.expect("Role is required for UserMessage"),
            content: self.content.expect("Content is required for UserMessage"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Text,
    JsonObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: ResponseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionRequest {
    pub model: Option<String>,
    pub messages: Vec<Message>,
    pub response_format: Option<ResponseFormat>,
    pub tools: Option<Vec<RequestTool>>,
}

impl ChatCompletionRequest {
    pub fn builder() -> ChatCompletionRequestBuilder {
        ChatCompletionRequestBuilder::new()
    }
}

pub struct ChatCompletionRequestBuilder {
    model: Option<String>,
    messages: Vec<Message>,
    response_format: Option<ResponseFormat>,
    tools: Vec<RequestTool>,
}

impl ChatCompletionRequestBuilder {
    pub fn new() -> Self {
        ChatCompletionRequestBuilder {
            model: None,
            messages: Vec::new(),
            response_format: None,
            tools: Vec::new(),
        }
    }
    pub fn model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn tool(mut self, request_tool: RequestTool) -> Self {
        self.tools.push(request_tool);
        self
    }

    pub fn build(self) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: self.model,
            messages: self.messages,
            response_format: self.response_format,
            tools: Some(self.tools),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolFunction {
    description: Option<String>,
    name: String,
    parameters: Option<serde_json::Value>,
}

impl ToolFunction {
    pub fn builder() -> ToolFunctionBuilder {
        ToolFunctionBuilder::new()
    }
}

pub struct ToolFunctionBuilder {
    description: Option<String>,
    name: Option<String>,
    parameters: Option<serde_json::Value>,
}

impl ToolFunctionBuilder {
    pub fn new() -> Self {
        ToolFunctionBuilder {
            description: None,
            name: None,
            parameters: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn parameters(mut self, parameters: serde_json::Value) -> Self {
        self.parameters = Some(parameters);
        self
    }

    pub fn build(self) -> ToolFunction {
        ToolFunction {
            description: self.description,
            name: self.name.expect("Name is required for ToolFunction"),
            parameters: self.parameters,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTool {
    #[serde(rename = "type")]
    _type: String,
    function: ToolFunction,
}

impl RequestTool {
    pub fn builder() -> RequestToolBuilder {
        RequestToolBuilder::new()
    }
}

pub struct RequestToolBuilder {
    _type: Option<String>,
    function: Option<ToolFunction>,
}

impl RequestToolBuilder {
    pub fn new() -> Self {
        RequestToolBuilder {
            _type: Some("function".to_string()),
            function: None,
        }
    }

    pub fn _type(mut self, _type: String) -> Self {
        self._type = Some(_type);
        self
    }

    pub fn function(mut self, function: ToolFunction) -> Self {
        self.function = Some(function);
        self
    }

    pub fn build(self) -> RequestTool {
        RequestTool {
            _type: self._type.expect("Type is required for RequestTool"),
            function: self.function.expect("Function is required for RequestTool"),
        }
    }
}
