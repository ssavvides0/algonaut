/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2003 {
    /// Block header data.
    #[serde(rename = "block")]
    pub block: serde_json::Value,
    /// Optional certificate object. This is only included when the format is set to message pack.
    #[serde(rename = "cert", skip_serializing_if = "Option::is_none")]
    pub cert: Option<serde_json::Value>,
}

impl InlineResponse2003 {
    pub fn new(block: serde_json::Value) -> InlineResponse2003 {
        InlineResponse2003 { block, cert: None }
    }
}
