use actix_web::{get, web, Responder, HttpResponse};
use uuid::Uuid;
use crate::models::AgentDecision;

#[get("/agent/decision")]
pub async fn agent_decision(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let state = query.get("state").unwrap_or(&"idle".to_string());

    let decision = match state.as_str() {
        "hungry" => "search_for_food",
        "tired" => "go_to_sleep",
        "curious" => "explore_area",
        _ => "remain_idle",
    };

    let response = AgentDecision {
        agent_id: format!("cat-{}", Uuid::new_v4()),
        decision: decision.to_string(),
    };

    HttpResponse::Ok().json(response)
}
