use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub function: ToolCallFunction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolCallFunction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

pub struct MessageBuilder {
    role: Option<String>,
    content: Option<String>,
    name: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
            role: None,
            content: None,
            name: None,
            tool_calls: None,
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

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Message {
        Message {
            role: self.role.expect("Role is required for UserMessage"),
            content: self.content,
            name: self.name,
            tool_calls: self.tool_calls,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    tools: Option<Vec<RequestTool>>,
}

impl ChatCompletionRequestBuilder {
    pub fn new() -> Self {
        ChatCompletionRequestBuilder {
            model: None,
            messages: Vec::new(),
            response_format: None,
            tools: None,
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
        if self.tools.is_none() {
            self.tools = Some(Vec::new());
        }
        if let Some(ref mut tools) = self.tools {
            tools.push(request_tool);
        }
        self
    }

    pub fn build(self) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: self.model,
            messages: self.messages,
            response_format: self.response_format,
            tools: self.tools,
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
