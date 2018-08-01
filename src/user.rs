use account::NewRegistration;
extern crate bcrypt;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result;
use schema::users;

pub enum UserCreateError {
    DbError(diesel::result::Error),
    HashError(bcrypt::BcryptError),
}

impl From<diesel::result::Error> for UserCreateError {
    fn from(e: diesel::result::Error) -> UserCreateError {
        UserCreateError::DbError(e)
    }
}
impl From<bcrypt::BcryptError> for UserCreateError {
    fn from(e: bcrypt::BcryptError) -> UserCreateError {
        UserCreateError::HashError(e)
    }
}

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
    ) -> Result<User, UserCreateError> {
        let insert = NewUser {
            name: user.name.clone(),
            password_hash: hash(&user.password, DEFAULT_COST)?,
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
