use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Debug, Serialize)]
/// http://docs.opengeospatial.org/is/17-069r3/17-069r3.html#_api_landing_page
struct CoreLandingPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    links: Vec<ApiLink>,
}

#[derive(Debug, Serialize)]
/// http://schemas.opengis.net/ogcapi/features/part1/1.0/openapi/schemas/link.yaml
struct ApiLink {
    href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rel: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hreflang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
/// http://docs.opengeospatial.org/is/17-069r3/17-069r3.html#_declaration_of_conformance_classes
struct CoreConformsTo {
    conforms_to: Vec<String>,
}

#[derive(Debug, Serialize)]
/// http://docs.opengeospatial.org/is/17-069r3/17-069r3.html#_collections_
struct CoreCollections {
    links: Vec<ApiLink>,
    collections: Vec<CoreCollection>,
}

#[derive(Debug, Serialize)]
struct CoreCollection {}

fn absurl(req: &HttpRequest, path: &str) -> String {
    let conninfo = req.connection_info();
    format!("{}://{}{}", conninfo.scheme(), conninfo.host(), path)
}

async fn index(req: HttpRequest) -> HttpResponse {
    let root = CoreLandingPage {
        title: Some("Buildings in Bonn".to_string()),
        description: Some("Access to data about buildings in the city of Bonn via a Web API that conforms to the OGC API Features specification".to_string()),
        links: vec![ApiLink {
            href: absurl(&req, "/"),
            rel: Some("self".to_string()),
            type_: Some("application/json".to_string()),
            title: Some("this document".to_string()),
            hreflang: None,
            length: None
        },
        ApiLink {
            href: absurl(&req, "/conformance"),
            rel: Some("conformance".to_string()),
            type_: Some("application/json".to_string()),
            title: Some("OGC API conformance classes implemented by this server".to_string()),
            hreflang: None,
            length: None
        },
        ApiLink {
            href: absurl(&req, "/collections"),
            rel: Some("data".to_string()),
            type_: Some("application/json".to_string()),
            title: Some("Information about the feature collections".to_string()),
            hreflang: None,
            length: None
        }]
    };
    HttpResponse::Ok().json(root)
}

async fn conformance() -> HttpResponse {
    let root = CoreConformsTo {
        conforms_to: vec![
            "http://www.opengis.net/spec/ogcapi-features-1/1.0/conf/core".to_string(),
            "http://www.opengis.net/spec/ogcapi-features-1/1.0/conf/geojson".to_string(),
        ],
    };
    HttpResponse::Ok().json(root)
}

async fn collections(req: HttpRequest) -> HttpResponse {
    let root = CoreCollections {
        links: vec![ApiLink {
            href: absurl(&req, "/collections.json"),
            rel: Some("self".to_string()),
            type_: Some("application/json".to_string()),
            title: Some("this document".to_string()),
            hreflang: None,
            length: None,
        }],
        collections: vec![],
    };
    HttpResponse::Ok().json(root)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/conformance").route(web::get().to(conformance)))
            .service(web::resource("/collections").route(web::get().to(collections)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let mut app =
            test::init_service(App::new().service(web::resource("/").route(web::get().to(index))))
                .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, "{\"title\":\"Buildings in Bonn\",\"description\":\"Access to data about buildings in the city of Bonn via a Web API that conforms to the OGC API Features specification\",\"links\":[{\"href\":\"http://data.example.org/\",\"rel\":\"self\",\"type\":\"application/json\",\"title\":\"this document\"},{\"href\":\"http://data.example.org/conformance\",\"rel\":\"conformance\",\"type\":\"application/json\",\"title\":\"OGC API conformance classes implemented by this server\"},{\"href\":\"http://data.example.org/collections\",\"rel\":\"data\",\"type\":\"application/json\",\"title\":\"Information about the feature collections\"}]}");

        Ok(())
    }
}
