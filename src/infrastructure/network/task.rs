use std::collections::HashMap;

use serde_json::Value;

/// Represents different types of network request tasks that can be performed.
/// 
/// This enum provides variants for handling various request formats:
/// - Plain text requests
/// - JSON requests
/// - Parameter-based requests
#[derive(Debug, Clone)]
pub enum NetworkTask {
    /// A plain text request without any specific data format
    RequestPlain,
    
    /// A JSON request containing structured data
    RequestJson(Value),
    
    /// A request with key-value parameters
    RequestParameters(HashMap<String, String>),
}
