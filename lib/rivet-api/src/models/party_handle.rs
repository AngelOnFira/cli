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
pub struct PartyHandle {
	#[serde(rename = "activity")]
	pub activity: Box<crate::models::PartyActivity>,
	#[serde(rename = "create_ts")]
	pub create_ts: String,
	#[serde(rename = "external")]
	pub external: Box<crate::models::PartyExternalLinks>,
	#[serde(rename = "party_id")]
	pub party_id: uuid::Uuid,
}

impl PartyHandle {
	pub fn new(
		activity: crate::models::PartyActivity,
		create_ts: String,
		external: crate::models::PartyExternalLinks,
		party_id: uuid::Uuid,
	) -> PartyHandle {
		PartyHandle {
			activity: Box::new(activity),
			create_ts,
			external: Box::new(external),
			party_id,
		}
	}
}
