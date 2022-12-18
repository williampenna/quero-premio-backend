use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;

#[get("/users", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn insert_new(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

#[get("/getUser?<id>", format = "application/json")]
pub fn get_by_username(conn: DbConn, id: String) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_username(id, &conn),
    }))
}