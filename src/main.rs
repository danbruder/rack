#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
use rocket::request::FlashMessage;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};
mod client;
mod user;
use client::Client;
use user::User;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate r2d2;
extern crate r2d2_diesel;
mod account;
mod db;
mod schema;
extern crate tera;
use tera::Context;

#[get("/")]
fn home(flash: Option<FlashMessage>, connection: db::Connection) -> Template {
    let mut context = Context::new();
    context.add("users", &User::read(&connection));
    context.add("register_path", &"/register".to_string());

    if let Some(msg) = flash {
        context.add("flash", &msg.msg());
    }
    Template::render("index", context)
}

#[get("/<path..>", rank = 5)]
fn assets(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![home, account::register, account::register_post, assets],
        ).launch();
}
