/*
 * A sample API conforming to the draft standard OGC API - Features - Part 1: Core
 *
 * This is a sample OpenAPI definition that conforms to the conformance classes \"Core\", \"GeoJSON\", \"HTML\" and \"OpenAPI 3.0\" of the draft standard \"OGC API - Features - Part 1: Core\".  This example is a generic OGC API Features definition that uses path parameters to describe all feature collections and all features. The generic OpenAPI definition does not provide any details on the collections or the feature content. This information is only available from accessing the feature collection resources.  There is [another example](ogcapi-features-1-example2.yaml) that specifies each collection explicitly.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@example.org
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct DataApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DataApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DataApiClient<C> {
        DataApiClient {
            configuration,
        }
    }
}

pub trait DataApi {
    fn get_feature(&self, collection_id: &str, feature_id: &str) -> Box<dyn Future<Item = crate::models::FeatureGeoJson, Error = Error<serde_json::Value>>>;
    fn get_features(&self, collection_id: &str, limit: Option<i32>, bbox: Option<Vec<f32>>, datetime: Option<&str>) -> Box<dyn Future<Item = crate::models::FeatureCollectionGeoJson, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>DataApi for DataApiClient<C> {
    fn get_feature(&self, collection_id: &str, feature_id: &str) -> Box<dyn Future<Item = crate::models::FeatureGeoJson, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/collections/{collectionId}/items/{featureId}".to_string())
        ;
        req = req.with_path_param("collectionId".to_string(), collection_id.to_string());
        req = req.with_path_param("featureId".to_string(), feature_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_features(&self, collection_id: &str, limit: Option<i32>, bbox: Option<Vec<f32>>, datetime: Option<&str>) -> Box<dyn Future<Item = crate::models::FeatureCollectionGeoJson, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/collections/{collectionId}/items".to_string())
        ;
        if let Some(ref s) = limit {
            req = req.with_query_param("limit".to_string(), s.to_string());
        }
        if let Some(ref s) = bbox {
            req = req.with_query_param("bbox".to_string(), s.join(",").to_string());
        }
        if let Some(ref s) = datetime {
            req = req.with_query_param("datetime".to_string(), s.to_string());
        }
        req = req.with_path_param("collectionId".to_string(), collection_id.to_string());

        req.execute(self.configuration.borrow())
    }

}
