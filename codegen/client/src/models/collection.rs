/*
 * A sample API conforming to the draft standard OGC API - Features - Part 1: Core
 *
 * This is a sample OpenAPI definition that conforms to the conformance classes \"Core\", \"GeoJSON\", \"HTML\" and \"OpenAPI 3.0\" of the draft standard \"OGC API - Features - Part 1: Core\".  This example is a generic OGC API Features definition that uses path parameters to describe all feature collections and all features. The generic OpenAPI definition does not provide any details on the collections or the feature content. This information is only available from accessing the feature collection resources.  There is [another example](ogcapi-features-1-example2.yaml) that specifies each collection explicitly.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@example.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    /// identifier of the collection used, for example, in URIs
    #[serde(rename = "id")]
    pub id: String,
    /// human readable title of the collection
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// a description of the features in the collection
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "links")]
    pub links: Vec<crate::models::Link>,
    #[serde(rename = "extent", skip_serializing_if = "Option::is_none")]
    pub extent: Option<crate::models::Extent>,
    /// indicator about the type of the items in the collection (the default value is 'feature').
    #[serde(rename = "itemType", skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    /// the list of coordinate reference systems supported by the service
    #[serde(rename = "crs", skip_serializing_if = "Option::is_none")]
    pub crs: Option<Vec<String>>,
}

impl Collection {
    pub fn new(id: String, links: Vec<crate::models::Link>) -> Collection {
        Collection {
            id,
            title: None,
            description: None,
            links,
            extent: None,
            item_type: None,
            crs: None,
        }
    }
}


