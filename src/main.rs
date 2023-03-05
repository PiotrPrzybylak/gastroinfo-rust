#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};


#[get("/")]
fn index() -> &'static str {
    "Hello, world with Rocket!"
}

#[get("/test")]
fn index2() -> Template {
    Template::render("index", context! { field: "value" })
}


#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing()).mount("/", routes![index, index2])
}