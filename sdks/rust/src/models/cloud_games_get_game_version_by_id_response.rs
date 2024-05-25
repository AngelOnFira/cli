/*
 * Rivet API EE
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudGamesGetGameVersionByIdResponse {
    #[serde(rename = "version")]
    pub version: Box<crate::models::CloudVersionFull>,
}

impl CloudGamesGetGameVersionByIdResponse {
    pub fn new(version: crate::models::CloudVersionFull) -> CloudGamesGetGameVersionByIdResponse {
        CloudGamesGetGameVersionByIdResponse {
            version: Box::new(version),
        }
    }
}

