use serde::Serialize;

#[derive(Debug, Serialize)]
/// http://docs.opengeospatial.org/is/17-069r3/17-069r3.html#_api_landing_page
pub struct CoreLandingPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub links: Vec<ApiLink>,
}

#[derive(Debug, Serialize)]
/// http://schemas.opengis.net/ogcapi/features/part1/1.0/openapi/schemas/link.yaml
pub struct ApiLink {
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
/// http://docs.opengeospatial.org/is/17-069r3/17-069r3.html#_declaration_of_conformance_classes
pub struct CoreConformsTo {
    pub conforms_to: Vec<String>,
}

#[derive(Debug, Serialize)]
/// http://docs.opengeospatial.org/is/17-069r3/17-069r3.html#_collections_
pub struct CoreCollections {
    pub links: Vec<ApiLink>,
    pub collections: Vec<CoreCollection>,
}

#[derive(Debug, Serialize)]
pub struct CoreCollection {}
