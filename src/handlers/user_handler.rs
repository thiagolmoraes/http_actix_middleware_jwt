use actix_web::{web, HttpResponse, routes};

use crate::{
    services::login,
    models::{LoginJson, UserClaim}
};



#[routes]
#[post("/login")]
pub async fn login_handler(credential: web::Json<LoginJson>) -> HttpResponse {

    match login(credential.into_inner()).await {
        Ok(user_session) => {

            match UserClaim::create_token(user_session.email){
                Ok(token) => {
                    HttpResponse::Ok().body(format!("session created: {}", token))
                },
                Err(error) => {
                    return HttpResponse::InternalServerError().body(error.to_string());
                }
            }
           
        },
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }

}

// Protected routes
#[routes]
#[get("/")]
pub async fn index() -> HttpResponse {

    HttpResponse::Ok().body("Hello to Initial Rust API")
}

#[routes]
#[get("/me")]
pub async fn whoami() -> HttpResponse {
    
        HttpResponse::Ok().body("Hello Fulano")

}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(login_handler);
    cfg.service(whoami);
}