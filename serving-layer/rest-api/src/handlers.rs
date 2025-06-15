use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct RouteQuery {
    origin: String,
    destination: String,
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({ "status": "ok" }))
}

pub async fn get_route_count(query: web::Query<RouteQuery>) -> impl Responder {
    let route = format!("{}-{}", query.origin, query.destination);
    // Here you would connect to Cassandra or Elasticsearch
    // Placeholder response:
    let result = json!({ "route": route, "flight_count": 123 });
    HttpResponse::Ok().json(result)
}