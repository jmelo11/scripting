use lefi::prelude::*;
use lefi::utils::errors::Result;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::response::status::{BadRequest, Custom};
use rocket::serde::json::Json;
use rocket::{catch, launch, post, routes};
use rocket::{catchers, Request, Response};
use serde::Serialize;

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        // Handle preflight OPTIONS request
        if request.method() == rocket::http::Method::Options {
            response.set_status(Status::Ok);
        }
    }
}

#[post("/execute", format = "application/json", data = "<event_stream>")]
fn execute(
    event_stream: Json<Vec<CodedEvent>>,
) -> std::result::Result<Json<Vec<Value>>, BadRequest<Json<ResponseError>>> {
    let events: Result<EventStream> = event_stream.into_inner().try_into();

    // Handle invalid events
    let events = match events {
        Ok(events) => events,
        Err(e) => {
            return Err(BadRequest(Json(ResponseError {
                status: Status::BadRequest,
                message: e.to_string(),
            })))
        }
    };

    // Index expressions and initialize evaluator (adjust according to your actual logic)
    let indexer = EventIndexer::new();
    indexer.visit_events(&events).unwrap();

    let scenarios = vec![];
    let evaluator =
        EventStreamEvaluator::new(indexer.get_variables_size()).with_scenarios(&scenarios);
    let results = evaluator.visit_events(&events);

    // Handle evaluation errors
    match results {
        Ok(results) => Ok(Json(results)),
        Err(e) => {
            return Err(BadRequest(Json(ResponseError {
                status: Status::BadRequest,
                message: e.to_string(),
            })))
        }
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![execute]) // Mount the OPTIONS route
        .register("/", catchers![invalid_entity])
}

#[derive(Serialize)]
pub struct ResponseError {
    pub status: Status,
    pub message: String,
}

#[catch(422)]
fn invalid_entity(request: &Request) -> Custom<Json<ResponseError>> {
    Custom(
        Status::UnprocessableEntity,
        Json(ResponseError {
            status: Status::UnprocessableEntity,
            message: format!("Invalid entity @ {}", request.uri()),
        }),
    )
}
