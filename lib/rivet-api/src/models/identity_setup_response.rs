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
pub struct IdentitySetupResponse {
	#[serde(rename = "game_id")]
	pub game_id: uuid::Uuid,
	#[serde(rename = "identity")]
	pub identity: Box<crate::models::IdentityProfile>,
	/// Documentation at https://jwt.io/
	#[serde(rename = "identity_token")]
	pub identity_token: String,
	/// If this token is comprimised, anyone with access to this token has control of the identity.
	#[serde(rename = "identity_token_expire_ts")]
	pub identity_token_expire_ts: String,
}

impl IdentitySetupResponse {
	pub fn new(
		game_id: uuid::Uuid,
		identity: crate::models::IdentityProfile,
		identity_token: String,
		identity_token_expire_ts: String,
	) -> IdentitySetupResponse {
		IdentitySetupResponse {
			game_id,
			identity: Box::new(identity),
			identity_token,
			identity_token_expire_ts,
		}
	}
}
