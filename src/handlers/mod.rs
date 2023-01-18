use actix_web::{web, web::ServiceConfig, HttpResponse};

pub fn app_config(config: &mut ServiceConfig) {
    let health_resource = web::resource("/").route(web::get().to(health));

    config.service(health_resource);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"success": true, "message": "Server is running and healthy...", "data": null}))
}
