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
pub struct FeatureGeoJson {
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "geometry")]
    pub geometry: crate::models::GeometryGeoJson,
    #[serde(rename = "properties")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::models::OneOfstringinteger>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<crate::models::Link>>,
}

impl FeatureGeoJson {
    pub fn new(_type: Type, geometry: crate::models::GeometryGeoJson, properties: Option<serde_json::Value>) -> FeatureGeoJson {
        FeatureGeoJson {
            _type,
            geometry,
            properties,
            id: None,
            links: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Feature")]
    Feature,
}

