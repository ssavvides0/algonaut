/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

/// DryrunSource : DryrunSource is TEAL source text that gets uploaded, compiled, and inserted into transactions or application state.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DryrunSource {
    #[serde(rename = "app-index")]
    pub app_index: i32,
    /// FieldName is what kind of sources this is. If lsig then it goes into the transactions[this.TxnIndex].LogicSig. If approv or clearp it goes into the Approval Program or Clear State Program of application[this.AppIndex].
    #[serde(rename = "field-name")]
    pub field_name: String,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "txn-index")]
    pub txn_index: i32,
}

impl DryrunSource {
    /// DryrunSource is TEAL source text that gets uploaded, compiled, and inserted into transactions or application state.
    pub fn new(app_index: i32, field_name: String, source: String, txn_index: i32) -> DryrunSource {
        DryrunSource {
            app_index,
            field_name,
            source,
            txn_index,
        }
    }
}
