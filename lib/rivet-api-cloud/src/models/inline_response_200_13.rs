/*
 * Rivet Cloud
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse20013 {
    #[serde(rename = "did_remove")]
    pub did_remove: bool,
}

impl InlineResponse20013 {
    pub fn new(did_remove: bool) -> InlineResponse20013 {
        InlineResponse20013 {
            did_remove,
        }
    }
}

