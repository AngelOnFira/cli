/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KvPutBatchRequest {
	#[serde(rename = "entries")]
	pub entries: Vec<crate::models::KvPutEntry>,
	#[serde(rename = "namespace_id", skip_serializing_if = "Option::is_none")]
	pub namespace_id: Option<uuid::Uuid>,
}

impl KvPutBatchRequest {
	pub fn new(entries: Vec<crate::models::KvPutEntry>) -> KvPutBatchRequest {
		KvPutBatchRequest {
			entries,
			namespace_id: None,
		}
	}
}
