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
pub struct EeCloudOpengbProjectsCreateRequest {
    #[serde(rename = "developer_group_id")]
    pub developer_group_id: uuid::Uuid,
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
}

impl EeCloudOpengbProjectsCreateRequest {
    pub fn new(developer_group_id: uuid::Uuid, display_name: String) -> EeCloudOpengbProjectsCreateRequest {
        EeCloudOpengbProjectsCreateRequest {
            developer_group_id,
            display_name,
        }
    }
}


