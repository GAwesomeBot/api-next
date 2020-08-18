#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_cors;

mod routes;
mod lib;

use rocket_cors::{
    AllowedOrigins,
    Cors, CorsOptions
};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://localhost:3000/login",
    ]);

    CorsOptions {
        allowed_origins,
        ..Default::default()
    }
        .to_cors()
        .expect("ERR CORS BUILD")
}

#[get("/")]
fn index() -> &'static str {
    "OK"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            routes::auth::login,
            routes::auth::login_callback,
            routes::auth::guilds,
        ])
        .attach(make_cors())
        .launch();
}
