/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChatSendMessageBodyText : The text in the text message.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatSendMessageBodyText {
	#[serde(rename = "body")]
	pub body: String,
}

impl ChatSendMessageBodyText {
	/// The text in the text message.
	pub fn new(body: String) -> ChatSendMessageBodyText {
		ChatSendMessageBodyText { body }
	}
}
