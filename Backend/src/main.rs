use rocket::serde::{json::Json, Serialize};
use rocket::{get, launch, routes};

#[derive(Serialize)]
enum Status {
    Success,
    Error,
}
#[derive(Serialize)]
struct IndexResponse {
    message: String,
    status: Status,
}

#[get("/")]
fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        message: "Hello, world!".to_string(),
        status: Status::Success,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
