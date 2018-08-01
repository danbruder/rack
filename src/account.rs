use db;
use diesel::result::Error::DatabaseError;
use rocket::request::FlashMessage;
use rocket::request::Form;
use rocket::response::status;
use rocket::response::Flash;
use rocket_contrib::Template;
use tera::Context;
use user::{NewUser, User};

#[derive(FromForm)]
pub struct NewRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[get("/register")]
fn register(flash: Option<FlashMessage>) -> Template {
    let mut context = Context::new();
    context.insert("register_path", &"/register".to_string());
    if let Some(msg) = flash {
        context.add("flash", &msg.msg());
    }
    Template::render("register", context)
}

#[post("/register", data = "<user>")]
fn register_post(
    user: Form<NewRegistration>,
    connection: db::Connection,
) -> Result<Flash<String>, status::Custom<String>> {
    match User::register(user.get(), &connection) {
        Ok(user) => {
            let path = &"/";
            let command = format!("Turbolinks.clearCache(); Turbolinks.visit('{}')", path);
            Ok(Flash::success(
                command,
                format!("Successfully registered. Welcome, {}", &user.name),
            ))
        }
        Err(error) => {
            let path = &"/register";
            let command = format!("Turbolinks.clearCache(); Turbolinks.visit('{}')", path);
            Ok(Flash::error(command, format!("{}", &error)))
        }
    }
}
