use account::NewRegistration;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result;
use schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[table_name = "users"]
#[derive(FromForm, Deserialize, Insertable, AsChangeset)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn register(
        user: &NewRegistration,
        connection: &MysqlConnection,
    ) -> Result<User, result::Error> {
        //let hashed = match hash(&user.password, DEFAULT_COST) {
        //Ok(h) => h,
        //Err(err) => return Err(String::from("Could not hash password")),
        //};
        let insert = NewUser {
            name: user.name.clone(),
            password_hash: "hey".to_string(),
            email: user.email.clone(),
        };

        diesel::insert_into(users::table)
            .values(&insert)
            .execute(connection)?;

        Ok(users::table
            .order(users::id.desc())
            .first(connection)
            .unwrap())
    }

    pub fn create(user: NewUser, connection: &MysqlConnection) -> Result<User, result::Error> {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)?;

        Ok(users::table
            .order(users::id.desc())
            .first(connection)
            .unwrap())
    }

    pub fn read(connection: &MysqlConnection) -> Vec<User> {
        users::table
            .order(users::id.asc())
            .load::<User>(connection)
            .unwrap()
    }

    pub fn update(id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id))
            .set(&user)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
