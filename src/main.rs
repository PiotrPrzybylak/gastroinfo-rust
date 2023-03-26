#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};


use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};


#[derive(Database)]
#[database("lanczyki")]
struct Lanczyki(sqlx::PgPool);

struct Place {
    _id: i64,
    name: String,
    zone: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world with Rocket!"
}

#[get("/test")]
async fn index2(mut db: Connection<Lanczyki>) -> Template {

    // let a: Option<String> = sqlx::query("SELECT id, name, zone FROM places")
    let ids = sqlx::query("SELECT name FROM places where id = 1")
    .fetch_one(&mut *db).await
    .and_then(|r| 
        {
            print!("duoa");
            let z= r.try_get(0);

           let x :String =  z.unwrap();
            print!("-----{}------", x);
            Ok(x)})
    .ok();

let dupa = ids.unwrap();

print!("{}", dupa);

    // let account = sqlx::query("select (1) as id, 'Herp Derpinson' as name")
    // .fetch_one(&mut db)
    // .await?;

// print!("{}", account);

        // .fetch(&mut *db)
        // .map_ok(|record| record.id)
        // .try_collect::<Vec<_>>()
        // .await.unwrap();

    // println!("{}", ids);


    // for row in client.query("SELECT id, name, zone FROM places", &[]).expect("DB SELECT error") {
    //     let place = Place {
    //         _id: row.get(0),
    //         name: row.get(1),
    //         zone: row.get(2),
    //     };
    //     println!("Author {} is from {}", place.name, place.zone);
    // }


    Template::render("index", context! { field:  dupa})
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Lanczyki::init())
        .mount("/", routes![index, index2])
}