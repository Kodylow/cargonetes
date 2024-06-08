use std::collections::BTreeMap;

use crate::utils::{
    deserialize_instant, deserialize_option_instant, serialize_instant, serialize_option_instant,
};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum State {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub state: State,
    pub image: String,
    pub memory: usize,
    pub disk: usize,
    pub exposed_ports: Vec<u16>,
    pub port_bindings: BTreeMap<String, String>,
    pub restart_policy: String,
    #[serde(
        serialize_with = "serialize_instant",
        deserialize_with = "deserialize_instant"
    )]
    pub start_time: Instant,
    #[serde(
        serialize_with = "serialize_option_instant",
        deserialize_with = "deserialize_option_instant"
    )]
    pub finish_time: Option<Instant>,
}

impl Task {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        state: State,
        image: String,
        memory: usize,
        disk: usize,
    ) -> Self {
        Self {
            id,
            name,
            state,
            image,
            memory,
            disk,
            exposed_ports: Vec::new(),
            port_bindings: BTreeMap::new(),
            restart_policy: "no".to_string(),
            start_time: Instant::now(),
            finish_time: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
    pub id: uuid::Uuid,
    pub state: State,
    #[serde(
        serialize_with = "serialize_instant",
        deserialize_with = "deserialize_instant"
    )]
    pub timestamp: Instant,
    pub task: Task,
}

impl TaskEvent {
    pub fn new(id: uuid::Uuid, state: State, timestamp: Instant, task: Task) -> Self {
        Self {
            id,
            state,
            timestamp,
            task,
        }
    }
}
