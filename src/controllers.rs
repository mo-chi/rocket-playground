use super::models::{IndexResponse, UserResponse};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> Json<IndexResponse> {
    let response = IndexResponse {
        message: "Hello, World!!".to_string(),
    };

    Json(response)
}

#[get("/users")]
pub fn get_users() -> Json<Vec<UserResponse>> {
    let users = vec![UserResponse {
        id: 1,
        name: "alice".to_string(),
    }];

    Json(users)
}

#[get("/users/<id>")]
pub fn get_users_byid(id: u64) -> Json<UserResponse> {
    let user = UserResponse {
        id,
        name: "alice".to_string(),
    };

    Json(user)
}

#[post("/users")]
pub fn post_users() -> Json<UserResponse> {
    let user = UserResponse {
        id: 1,
        name: "alice".to_string(),
    };

    Json(user)
}
