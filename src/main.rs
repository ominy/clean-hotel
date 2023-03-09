#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} years old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[post("/helllo", data = "<name>")]
fn helllo(name: String) -> String {
    format!("This is my name {name}")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hello", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![helllo])
        .mount("/", FileServer::from("../ui/build/"))
}
