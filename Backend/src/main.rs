use rocket::serde::{json::Json, Serialize};
use rocket::{get, launch, routes};

#[derive(Serialize)]
enum Status {
    Success,
    Error,
}
#[derive(Serialize)]
struct IndexResponse {
    message: &'static str,
    status: Status,
}

#[get("/")]
fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        message: "Hello, world!",
        status: Status::Success,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
