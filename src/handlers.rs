use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use std::time::Instant;
use log::info;
use crate::models::{AgentState, AgentDecision, Behavior};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    state: Option<String>,
}

#[get("/agent/decision")]
pub async fn agent_decision(query: web::Query<QueryParams>) -> impl Responder {
    let req_id = Uuid::new_v4();
    let start = Instant::now();

    let state_str = query.state.clone().unwrap_or_else(|| "unknown".to_string());
    let parsed_state = state_str.parse().unwrap_or(AgentState::Unknown);

    info!("🔍 [{}] Received state: {:?}", req_id, parsed_state);

    let decision = parsed_state.decide();

    let response = AgentDecision {
        request_id: req_id.to_string(),
        agent_id: format!("cat-{}", Uuid::new_v4()),
        decision: decision.to_string(),
        duration_ms: start.elapsed().as_millis(),
    };

    HttpResponse::Ok().json(response)
}
