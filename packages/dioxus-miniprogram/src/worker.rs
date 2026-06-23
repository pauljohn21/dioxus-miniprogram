//! Worker communication module for Mini Program
//! 
//! This module handles communication between the main thread and the Worker
//! that runs the Dioxus WASM code.

use serde::{Deserialize, Serialize};
use serde_json;

/// Message types for Worker communication
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerMsgType {
    /// Initialize the WASM module
    Init,
    /// Call a WASM function
    Call,
    /// Result from Worker
    Result,
    /// Error from Worker
    Error,
    /// DOM operation from Worker
    DomOp,
}

/// A message sent to or from the Worker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMessage {
    /// Message type
    #[serde(rename = "type")]
    pub msg_type: WorkerMsgType,
    /// Call ID for matching requests and responses
    pub call_id: Option<u32>,
    /// Function name to call (for Call messages)
    pub function_name: Option<String>,
    /// Arguments for the function call
    pub args: Option<Vec<serde_json::Value>>,
    /// Result value
    pub result: Option<serde_json::Value>,
    /// Error message
    pub error: Option<String>,
    /// DOM operation data
    pub data: Option<serde_json::Value>,
}

/// Create a new init message
pub fn init_message() -> WorkerMessage {
    WorkerMessage {
        msg_type: WorkerMsgType::Init,
        call_id: None,
        function_name: None,
        args: None,
        result: None,
        error: None,
        data: None,
    }
}

/// Create a new call message
pub fn call_message(call_id: u32, function_name: &str, args: Vec<serde_json::Value>) -> WorkerMessage {
    WorkerMessage {
        msg_type: WorkerMsgType::Call,
        call_id: Some(call_id),
        function_name: Some(function_name.to_string()),
        args: Some(args),
        result: None,
        error: None,
        data: None,
    }
}
