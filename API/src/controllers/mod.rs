use actix_web::web;

pub mod auth;
pub mod json;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/login").route(web::post().to(auth::login)))
        .service(web::resource("/register").route(web::post().to(auth::register)))
        .service(web::resource("/json").route(web::post().to(json::process_json)));
}
