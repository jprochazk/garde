//! Simple usage of using `axum_garde` for a REST API
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example json --features=json
//! ```
use axum::{response::IntoResponse, routing::post, Json, Router, Server};
use axum_garde::WithValidation;
use garde::Validate;
use serde::{Deserialize, Serialize};

// Define your valid scheme
#[derive(Debug, Serialize, Deserialize, Validate)]
struct Person {
    #[garde(ascii, length(min = 3, max = 25))]
    username: String,
    #[garde(length(min = 15))]
    password: String,
}

async fn insert_valid_person(
    // Perform validation on the request payload
    WithValidation(person): WithValidation<Json<Person>>,
) -> impl IntoResponse {
    println!("Inserted person on database: {person:?}");
    Json(person.into_inner())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/person", post(insert_valid_person))
        // Create the application state
        .with_state(());
    println!("See example: http://127.0.0.1:8080/person");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
