#[macro_use] extern crate rocket;
#[macro_use] extern crate maplit;

mod routes;

use std::collections::HashMap;

use rocket::{Build, Rocket};

use rocket::serde::{json::Json, Serialize};

#[get("/")]
fn index() -> Json<HashMap<String, String>> {
    "Hello, world!";
    let mut result_dict = HashMap::new();
    let result_value = true;
    result_dict.insert(
        "result".to_string(),
        result_value.to_string(),
    );

    Json(result_dict)
}


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .attach(routes::health::stage())
}