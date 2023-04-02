//! Showcases custom validators and garde contexts
//!
//! Run the example using
//!
//! ```sh
//! cargo run --example custom_validation --features=json
//! ```
use axum::{response::IntoResponse, routing::post, Json, Router, Server};
use axum_garde::WithValidation;
use garde::Validate;
use serde::{Deserialize, Serialize};

// Define your valid scheme
#[derive(Debug, Serialize, Deserialize, Validate)]
#[garde(context(PasswordContext))]
struct Person {
    #[garde(ascii, length(min = 3, max = 25))]
    username: String,
    #[garde(custom(password_validation))]
    password: String,
}

// Define your custom context
#[derive(Debug, Clone)]
struct PasswordContext {
    complexity: usize,
}

// Define your custom validation
fn password_validation(value: &str, context: &PasswordContext) -> garde::Result {
    if value.len() < context.complexity {
        return Err(garde::Error::new("password is not strong enough"));
    }
    Ok(())
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
        .with_state(PasswordContext { complexity: 10 });
    println!("See example: http://127.0.0.1:8080/person");
    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
