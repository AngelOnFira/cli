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
pub struct IdentityWatchEventsResponse {
	#[serde(rename = "events")]
	pub events: Vec<crate::models::IdentityGlobalEvent>,
	#[serde(rename = "watch")]
	pub watch: Box<crate::models::WatchResponse>,
}

impl IdentityWatchEventsResponse {
	pub fn new(
		events: Vec<crate::models::IdentityGlobalEvent>,
		watch: crate::models::WatchResponse,
	) -> IdentityWatchEventsResponse {
		IdentityWatchEventsResponse {
			events,
			watch: Box::new(watch),
		}
	}
}
