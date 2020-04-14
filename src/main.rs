#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;
mod lib;

#[get("/")]
fn index() -> &'static str {
    "OK"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            routes::auth::login,
            routes::auth::login_callback
        ])
        .launch();
}
