use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;
use crate::models::UserData;

#[post("/users", format = "application/json")]
pub fn getAll(conn: DbConn) -> Json<Value> {
    let users = User::getAllUsers(&conn);
    Json(json!({
        "status": 200,
        "result": "users",
    }))
}

#[post("/users", format = "application/json", data = "<newUser")]
pub fn newUser(conn: DbConn, newUser: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insertUser(newUser.into_inner(), &conn),
        "result": User::getAllUsers(&conn).first(),
    }))
}

#[post("/users", format = "application/json", data = "<user_data")]
pub fn findUser(conn: DbConn, user_data: Json<UserData>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::getUserByUsername(userData.into_inner(), &conn),
    }))
}