use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/simulate", post(simulate));

    // run our app with hyper
    // axum 0.7 uses tokio::net::TcpListener
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Welcome to PiKOnE!"
}

#[derive(Deserialize)]
struct SimulationRequest {
    choice_a: String,
    choice_b: String,
    sessions: usize,
}

#[derive(Serialize)]
struct SimulationResponse {
    z_score: f64,
    message: String,
}

async fn simulate(Json(payload): Json<SimulationRequest>) -> Json<SimulationResponse> {
    // Placeholder logic for simulation
    // In a real implementation, this would interact with CURBy and perform the loop described in README

    // Simulate some result
    let z_score = 0.0; // Placeholder

    Json(SimulationResponse {
        z_score,
        message: format!("Simulated {} sessions for choices {} vs {}", payload.sessions, payload.choice_a, payload.choice_b),
    })
}
