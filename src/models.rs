use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::{Timestamp, Date};
use super::schema::usuario;
use super::schema::usuario::dsl::usuario as allUsers;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub cpf: String,
    pub creation_time: Timestamp,
    pub update_time: Timestamp,
    pub date_of_birth: String,
}

// Decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
}

// This is to insert users to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "usuario"]
pub struct NewUser {
    pub nome: String,
    pub cpf: String,
    pub data_nascimento: Date,
}

impl User {
    pub fn getAllUsers(conn: &PgConnection) -> Vec<User> {
        allUsers
            .order(User::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insertUser(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(usuario::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn getUserByUsername (user: UserData, conn: &PgConnection) -> Vec<User> {
        allUsers
            .filter(User::nome.eq(user.name))
    }
}