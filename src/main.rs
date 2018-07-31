#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
use rocket_contrib::{Json, Value};
mod client;
use client::{NewClient, Client};
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
mod db;
mod schema;

#[post("/", data = "<client>")]
fn create(client: Json<NewClient>, connection: db::Connection) -> Json<Client> {
    let insert = NewClient { ..client.into_inner() };
    Json(Client::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Client::read(&connection)))
}

#[put("/<id>", data = "<client>")]
fn update(id: i32, client: Json<Client>, connection: db::Connection) -> Json<Value> {
    let update = Client { id: id, ..client.into_inner() };
    Json(json!({
        "success": Client::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Client::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![create, read, update, delete])
        .launch();
}
