/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudCdnSiteSummary : A CDN site summary.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudCdnSiteSummary {
    /// Whether or not this site has completely been uploaded.
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Unsigned 64 bit integer.
    #[serde(rename = "content_length", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// RFC3339 timestamp.
    #[serde(rename = "create_ts")]
    pub create_ts: String,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A universally unique identifier.
    #[serde(rename = "site_id")]
    pub site_id: String,
    /// A universally unique identifier.
    #[serde(rename = "upload_id")]
    pub upload_id: String,
}

impl CloudCdnSiteSummary {
    /// A CDN site summary.
    pub fn new(create_ts: String, display_name: String, site_id: String, upload_id: String) -> CloudCdnSiteSummary {
        CloudCdnSiteSummary {
            complete: None,
            content_length: None,
            create_ts,
            display_name,
            site_id,
            upload_id,
        }
    }
}


