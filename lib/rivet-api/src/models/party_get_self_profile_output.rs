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
pub struct PartyGetSelfProfileOutput {
    #[serde(rename = "party", skip_serializing_if = "Option::is_none")]
    pub party: Option<Box<crate::models::PartyProfile>>,
    #[serde(rename = "watch")]
    pub watch: Box<crate::models::WatchResponse>,
}

impl PartyGetSelfProfileOutput {
    pub fn new(watch: crate::models::WatchResponse) -> PartyGetSelfProfileOutput {
        PartyGetSelfProfileOutput {
            party: None,
            watch: Box::new(watch),
        }
    }
}

