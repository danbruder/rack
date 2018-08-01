#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
use rocket::request::{FlashMessage, Form};
use rocket::response::NamedFile;
use rocket::response::{Flash, Redirect};
use rocket_contrib::{Json, Template, Value};
use std::path::{Path, PathBuf};
mod client;
use client::{Client, NewClient};
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
mod db;
use std::collections::{BTreeMap, HashMap};
mod schema;
extern crate tera;
use rocket::response::status;
use tera::Context;

#[post("/", data = "<client>")]
fn create(client: Json<NewClient>, connection: db::Connection) -> Json<Client> {
    let insert = NewClient {
        ..client.into_inner()
    };
    Json(Client::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Client::read(&connection)))
}

#[put("/<id>", data = "<client>")]
fn update(id: i32, client: Json<Client>, connection: db::Connection) -> Json<Value> {
    let update = Client {
        id: id,
        ..client.into_inner()
    };
    Json(json!({
        "success": Client::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({ "success": Client::delete(id, &connection) }))
}

// Registration
#[get("/")]
fn home(flash: Option<FlashMessage>, connection: db::Connection) -> Template {
    let mut context = Context::new();
    context.add("clients", &Client::read(&connection));
    context.add("register_path", &"/register".to_string());

    if let Some(msg) = flash {
        context.add("flash", &msg.msg());
    }
    Template::render("index", context)
}

// Registration
#[get("/register")]
fn register() -> Template {
    let mut context = Context::new();
    context.insert("register_path", &"/register".to_string());
    Template::render("register", context)
}

#[post("/register", data = "<client>")]
fn register_post(
    client: Form<NewClient>,
    connection: db::Connection,
) -> Result<Flash<String>, status::Custom<String>> {
    // Create the thing
    let insert = NewClient {
        ..client.into_inner()
    };

    Client::create(insert, &connection);

    let path = &"/";
    let command = format!("Turbolinks.clearCache(); Turbolinks.visit('{}')", path);

    Ok(Flash::success(command, "Success"))
}

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .attach(Template::fairing())
        .mount("/", routes![home, register, register_post, all])
        .mount("/clients", routes![create, read, update, delete])
        .launch();
}
