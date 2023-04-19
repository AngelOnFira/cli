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
pub struct CloudGroupBillingCheckoutRequest {
	/// How much money to checkout (in hundred-thousandths USD, 100,000 = $1.00).
	#[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
	pub amount: Option<i64>,
}

impl CloudGroupBillingCheckoutRequest {
	pub fn new() -> CloudGroupBillingCheckoutRequest {
		CloudGroupBillingCheckoutRequest { amount: None }
	}
}
