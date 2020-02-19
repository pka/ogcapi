mod db;
mod ogcapi;
mod openapi;
#[cfg(test)]
mod tests;

use crate::ogcapi::*;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use deadpool_postgres::{Client, Pool};
use dotenv::dotenv;
use tokio_postgres::NoTls;

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
            href: absurl(&req, "/api"),
            rel: Some("service-desc".to_string()),
            type_: Some("application/vnd.oai.openapi+json;version=3.0".to_string()),
            title: Some("the API definition".to_string()),
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

// Test with https://editor.swagger.io/?url=http://localhost:8080/api
async fn api(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/vnd.oai.openapi+json;version=3.0")
        .body(openapi::OPEN_API_TEMPLATE.replace("https://data.example.org/", &absurl(&req, "/")))
}

async fn conformance() -> HttpResponse {
    let root = CoreConformsTo {
        conforms_to: vec![
            "http://www.opengis.net/spec/ogcapi-features-1/1.0/conf/core".to_string(),
            "http://www.opengis.net/spec/ogcapi-features-1/1.0/conf/oas30".to_string(),
            "http://www.opengis.net/spec/ogcapi-features-1/1.0/conf/geojson".to_string(),
        ],
    };
    HttpResponse::Ok().json(root)
}

async fn collections(req: HttpRequest) -> HttpResponse {
    let root = CoreCollections {
        links: vec![ApiLink {
            href: absurl(&req, "/collections"),
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

pub async fn db_query(db_pool: web::Data<Pool>) -> HttpResponse {
    let client: Client = db_pool.get().await.unwrap();

    let val = db::db_query(&client).await;

    HttpResponse::Ok().json(val)
}

pub mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new()).unwrap();
            cfg.try_into()
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::Config::from_env().expect("Config::from_env");
    let pool = config.pg.create_pool(NoTls).expect("create_pool");
    // Test connection
    pool.get().await.expect("Connection failed");

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/api").route(web::get().to(api)))
            .service(web::resource("/conformance").route(web::get().to(conformance)))
            .service(web::resource("/collections").route(web::get().to(collections)))
            .service(web::resource("/db").route(web::get().to(db_query)))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
