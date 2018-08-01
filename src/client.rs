use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use schema::clients;

#[table_name = "clients"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Client {
    pub id: i32,
    pub name: String,
}

#[table_name = "clients"]
#[derive(FromForm, Deserialize, Insertable, AsChangeset)]
pub struct NewClient {
    pub name: String,
}

impl Client {
    pub fn create(client: NewClient, connection: &MysqlConnection) -> Client {
        diesel::insert_into(clients::table)
            .values(&client)
            .execute(connection)
            .expect("Error creating new client");

        clients::table
            .order(clients::id.desc())
            .first(connection)
            .unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Client> {
        clients::table
            .order(clients::id.asc())
            .load::<Client>(connection)
            .unwrap()
    }

    pub fn update(id: i32, client: Client, connection: &MysqlConnection) -> bool {
        diesel::update(clients::table.find(id))
            .set(&client)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(clients::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
