#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
use rocket::response::{Flash, Redirect};
use rocket_contrib::{Json, Template, Value};
mod client;
use client::{Client, NewClient};
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
mod db;
use std::collections::{BTreeMap, HashMap};
mod schema;

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
fn home() -> Template {
    let mut context = BTreeMap::new();
    context.insert("register_path".to_string(), "/register".to_string());
    Template::render("index", context)
}

// Registration
#[get("/register")]
fn register() -> Template {
    let mut context = BTreeMap::new();
    context.insert("world".to_string(), "世界!".to_string());
    context.insert("register_path".to_string(), "/register".to_string());
    Template::render("register", context)
}

#[post("/register")]
fn register_post() -> Flash<Redirect> {
    Flash::new(Redirect::to("/"), "suggestion", "Try this out!")
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .attach(Template::fairing())
        .mount("/", routes![home, register, register_post])
        .mount("/clients", routes![create, read, update, delete])
        .launch();
}
