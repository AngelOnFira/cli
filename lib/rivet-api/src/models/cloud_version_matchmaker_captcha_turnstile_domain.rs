/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionMatchmakerCaptchaTurnstileDomain : Turnstile domain configuration.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionMatchmakerCaptchaTurnstileDomain {
	#[serde(rename = "secret_key")]
	pub secret_key: String,
}

impl CloudVersionMatchmakerCaptchaTurnstileDomain {
	/// Turnstile domain configuration.
	pub fn new(secret_key: String) -> CloudVersionMatchmakerCaptchaTurnstileDomain {
		CloudVersionMatchmakerCaptchaTurnstileDomain { secret_key }
	}
}
