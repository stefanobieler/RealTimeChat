use rocket::routes;

#[macro_use]
extern crate rocket;

#[get("/world")]
async fn world() -> &'static str {
    return "Hello, World";
}


#[rocket::main]
async fn main() {
    return rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await.unwrap();
}
