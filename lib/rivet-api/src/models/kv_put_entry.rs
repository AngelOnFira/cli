/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KvPutEntry : A new entry to insert into the key-value database.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct KvPutEntry {
    /// A string representing a key in the key-value database. Key path components are split by a slash (e.g. `a/b/c` has the path components `[\"a\", \"b\", \"c\"]`). Slashes can be escaped by using a forward slash (e.g. `a/b/c/d` has the path components `[\"a\", \"b/c\", \"d\"]`). See `rivet.api.kv.common#KeyComponents` for the structure of a `rivet.api.kv.common#Key` split by `/`.
    #[serde(rename = "key")]
    pub key: String,
}

impl KvPutEntry {
    /// A new entry to insert into the key-value database.
    pub fn new(key: String) -> KvPutEntry {
        KvPutEntry {
            key,
        }
    }
}


