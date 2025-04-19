use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct AgentDecision {
    pub request_id: String,
    pub agent_id: String,
    pub decision: String,
    pub duration_ms: u128,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentState {
    Hungry,
    Tired,
    Curious,
    Aggressive,
    Lazy,
    Unknown,
}

impl FromStr for AgentState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hungry" => Ok(Self::Hungry),
            "tired" => Ok(Self::Tired),
            "curious" => Ok(Self::Curious),
            "aggressive" => Ok(Self::Aggressive),
            "lazy" => Ok(Self::Lazy),
            _ => Ok(Self::Unknown),
        }
    }
}

pub trait Behavior {
    fn decide(&self) -> &'static str;
}

impl Behavior for AgentState {
    fn decide(&self) -> &'static str {
        match self {
            AgentState::Hungry => "search_for_food",
            AgentState::Tired => "go_to_sleep",
            AgentState::Curious => "explore_area",
            AgentState::Aggressive => "defend_territory",
            AgentState::Lazy => "stare_into_void",
            AgentState::Unknown => "remain_idle",
        }
    }
}
