use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime, Utc};

use super::schema::usuario;
use super::schema::usuario::dsl::usuario as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub nome: String,
    pub cpf: String,
    pub data_criacao: NaiveDateTime,
    pub data_atualizacao: NaiveDateTime,
    pub data_nascimento: NaiveDate,
}

#[derive(Serialize, Insertable)]
#[table_name = "usuario"]
struct UserToSave {
    pub nome: String,
    pub cpf: String,
    pub data_criacao: NaiveDateTime,
    pub data_atualizacao: NaiveDateTime,
    pub data_nascimento: NaiveDate,
}

// Decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
}

// This is to insert users to database
#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub nome: String,
    pub cpf: String,
    pub data_nascimento: NaiveDate,
}

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(usuario::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
        let naive_date_time = Utc::now().naive_utc();
        let user_to_save = UserToSave {
            nome: user.nome,
            cpf: user.cpf,
            data_criacao: naive_date_time,
            data_atualizacao: naive_date_time,
            data_nascimento: user.data_nascimento
        };
        diesel::insert_into(usuario::table)
            .values(user_to_save)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_username (id: String, conn: &PgConnection) -> Vec<User> {
        let id_to_search: i32 = id.parse().unwrap();
        all_users
            .filter(usuario::id.eq(id_to_search))
            .load::<User>(conn)
            .expect("error!")
    }
}