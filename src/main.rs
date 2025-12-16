use axum::{
    routing::{get, post},
    Router,
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, HeaderMap, header},
    extract::Path,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod curby;
mod simulation;
mod stats;
mod report;

use simulation::{SimulationEngine, SessionStep};

// In-memory state to hold active sessions
// In a production app, use a database or Redis.
struct AppState {
    // Map sessionId -> (Steps, OriginalConfig)
    sessions: Mutex<HashMap<String, Vec<SessionStep>>>,
    // Map sessionId -> PDF path
    reports: Mutex<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState {
        sessions: Mutex::new(HashMap::new()),
        reports: Mutex::new(HashMap::new()),
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/api/start", post(start_simulation))
        .route("/api/analyze", post(analyze_results))
        .route("/api/report/:session_id", get(download_report))
        .with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> Html<String> {
    match std::fs::read_to_string("index.html") {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>Error loading index.html</h1>".to_string()),
    }
}

#[derive(Deserialize)]
struct StartRequest {
    sessions: usize,
}

#[derive(Serialize)]
struct StartResponse {
    session_id: String,
    steps: Vec<ClientStep>,
}

#[derive(Serialize)]
struct ClientStep {
    left_number: u8,
    right_number: u8,
}

async fn start_simulation(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<StartRequest>,
) -> Result<Json<StartResponse>, (StatusCode, String)> {
    let mut engine = SimulationEngine::new();

    // Generate steps using CURBy
    let steps = engine.generate_sessions(payload.sessions)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Entropy Error: {}", e)))?;

    let session_id = uuid::Uuid::new_v4().to_string();

    // Store full steps (including hidden choice info)
    {
        let mut sessions_lock = state.sessions.lock().unwrap();
        sessions_lock.insert(session_id.clone(), steps.clone());
    }

    // Return only public info to client
    let client_steps: Vec<ClientStep> = steps.into_iter().map(|s| ClientStep {
        left_number: s.left_number,
        right_number: s.right_number,
    }).collect();

    Ok(Json(StartResponse {
        session_id,
        steps: client_steps,
    }))
}

#[derive(Deserialize)]
struct AnalyzeRequest {
    session_id: String,
    selections: Vec<String>, // "left" or "right"
    choice_a: String,
    choice_b: String,
}

#[derive(Serialize)]
struct AnalyzeResponse {
    z_score: f64,
    count_a: usize,
    count_b: usize,
    message: String,
}

async fn analyze_results(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, (StatusCode, String)> {
    let steps = {
        let sessions_lock = state.sessions.lock().unwrap();
        sessions_lock.get(&payload.session_id)
            .cloned()
            .ok_or((StatusCode::NOT_FOUND, "Session not found".to_string()))?
    };

    if steps.len() != payload.selections.len() {
        return Err((StatusCode::BAD_REQUEST, "Mismatch in session length".to_string()));
    }

    let mut count_a = 0;
    let mut count_b = 0;

    for (step, selection) in steps.iter().zip(payload.selections.iter()) {
        let selected_left = selection == "left";

        // logic:
        // step.is_choice_a_left == true AND selected_left == true => A
        // step.is_choice_a_left == true AND selected_left == false => B
        // step.is_choice_a_left == false AND selected_left == true => B
        // step.is_choice_a_left == false AND selected_left == false => A

        if step.is_choice_a_left == selected_left {
            count_a += 1;
        } else {
            count_b += 1;
        }
    }

    let z_score = stats::calculate_z_score(steps.len(), count_a);
    let message = if z_score.abs() > 1.96 {
        "Significant Anomaly Detected".to_string()
    } else {
        "No Significant Anomaly".to_string()
    };

    // Generate PDF
    let report_filename = format!("report_{}.pdf", payload.session_id);
    report::generate_pdf(
        &report_filename,
        &payload.choice_a,
        &payload.choice_b,
        steps.len(),
        count_a,
        count_b,
        z_score
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("PDF Error: {}", e)))?;

    // Store report path
    {
        let mut reports_lock = state.reports.lock().unwrap();
        reports_lock.insert(payload.session_id.clone(), report_filename);
    }

    Ok(Json(AnalyzeResponse {
        z_score,
        count_a,
        count_b,
        message,
    }))
}

async fn download_report(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Path(session_id): Path<String>,
) ->  Result<impl IntoResponse, (StatusCode, String)> {
    let filename = {
        let reports_lock = state.reports.lock().unwrap();
        reports_lock.get(&session_id)
            .cloned()
            .ok_or((StatusCode::NOT_FOUND, "Report not found".to_string()))?
    };

    let file = tokio::fs::File::open(&filename).await
        .map_err(|_| (StatusCode::NOT_FOUND, "File not found on disk".to_string()))?;

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = axum::body::Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", filename).parse().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid filename".to_string()))?
    );

    Ok((headers, body))
}
