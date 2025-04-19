use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AgentDecision {
    pub agent_id: String,
    pub decision: String,
}
