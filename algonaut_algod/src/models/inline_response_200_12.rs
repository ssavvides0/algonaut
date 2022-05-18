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
pub struct InlineResponse20012 {
    /// disassembled Teal code
    #[serde(rename = "result")]
    pub result: String,
}

impl InlineResponse20012 {
    pub fn new(result: String) -> InlineResponse20012 {
        InlineResponse20012 {
            result,
        }
    }
}


