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
pub struct InlineResponse20011 {
    /// base32 SHA512_256 of program bytes (Address style)
    #[serde(rename = "hash")]
    pub hash: String,
    /// base64 encoded program bytes
    #[serde(rename = "result")]
    pub result: String,
}

impl InlineResponse20011 {
    pub fn new(hash: String, result: String) -> InlineResponse20011 {
        InlineResponse20011 {
            hash,
            result,
        }
    }
}


