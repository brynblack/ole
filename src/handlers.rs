use actix_web::HttpResponse;

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("Logged in!")
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("Logged out!")
}

pub async fn create_acc() -> HttpResponse {
    HttpResponse::Ok().body("Created Account!")
}

pub async fn remove_acc() -> HttpResponse {
    HttpResponse::Ok().body("Deleted Account!")
}
