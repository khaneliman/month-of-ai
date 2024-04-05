use std::collections::HashMap;

use super::chat_completion_request::Message;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct OAIChoices {
    pub text: String,
    pub index: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logprobs: Option<u8>,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Deserialize, Debug)]
pub struct ChatCompletionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<u64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Usage,
    pub system_fingerprint: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageRole {
    user,
    system,
    assistant,
    function,
}

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
pub enum FinishReason {
    stop,
    length,
    content_filter,
    tool_calls,
    null,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinishDetails {
    #[serde(rename = "type")]
    pub _type: FinishReason,
    pub stop: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionChoice {
    pub index: i64,
    pub message: ChatCompletionMessageForResponse,
    pub finish_reason: Option<FinishReason>,
    pub finish_details: Option<FinishDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionMessageForResponse {
    pub role: MessageRole,
    pub content: Option<String>,
    pub name: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}
