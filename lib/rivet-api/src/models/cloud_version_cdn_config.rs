/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudVersionCdnConfig : CDN configuration for a given version.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudVersionCdnConfig {
    /// Multiple CDN version routes.
    #[serde(rename = "routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<crate::models::CloudVersionCdnRoute>>,
    /// A universally unique identifier.
    #[serde(rename = "site_id", skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

impl CloudVersionCdnConfig {
    /// CDN configuration for a given version.
    pub fn new() -> CloudVersionCdnConfig {
        CloudVersionCdnConfig {
            routes: None,
            site_id: None,
        }
    }
}


