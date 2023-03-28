#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket_db_pools::{sqlx, Database, Connection};
use rocket::{Rocket, Build, futures};
use futures::{stream::TryStreamExt, future::TryFutureExt};
use rocket::serde::{Serialize, Deserialize};


#[derive(Database)]
#[database("lanczyki")]
struct Lanczyki(sqlx::PgPool);


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Place {
    id: i64,
    name: String,
    zone: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world with Rocket!"
}

#[get("/test")]
async fn index2(mut db: Connection<Lanczyki>) -> Template {

    let places = sqlx::query!("SELECT id, name, zone FROM places")
        .fetch(&mut *db)
        .map_ok(|r|

                    Place {
                        id: r.id,
                        name: r.name,
                        zone: r.zone.unwrap(),
                    }

        )
        .try_collect::<Vec<_>>()
        .await.ok().unwrap();

    Template::render("index", context! { places: places})
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Lanczyki::init())
        .mount("/", routes![index, index2])
}