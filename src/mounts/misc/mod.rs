use actix_web::{ HttpResponse};

pub async fn heartbeat() -> HttpResponse {
    let tjson = json!({ "alive": "true" });
    return HttpResponse::Ok().json(tjson)
}
