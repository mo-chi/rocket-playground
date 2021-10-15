use super::controllers::*;

pub fn routes() -> Vec<rocket::Route> {
    routes![index, get_users, get_users_byid, post_users]
}
