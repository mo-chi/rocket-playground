#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod routes;

/// Rocket に URL のエンドポイントを設定する
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes::routes())
}

/// Rocket の起動
fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn index() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("{\"message\":\"Hello, World!!\"}".to_string())
        );
    }
}
