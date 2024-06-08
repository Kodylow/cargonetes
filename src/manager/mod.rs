use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::task::{Task, TaskEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manager {
    pub pending: VecDeque<Task>,
    pub task_db: BTreeMap<uuid::Uuid, Vec<Task>>,
    pub event_db: BTreeMap<uuid::Uuid, Vec<TaskEvent>>,
    pub workers: Vec<String>,
    pub worker_task_map: BTreeMap<String, Vec<Task>>,
    pub task_worker_map: BTreeMap<uuid::Uuid, String>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            task_db: BTreeMap::new(),
            event_db: BTreeMap::new(),
            workers: Vec::new(),
            worker_task_map: BTreeMap::new(),
            task_worker_map: BTreeMap::new(),
        }
    }
    pub fn select_worker(&self) {
        info!("I will select an appropriate worker");
    }

    pub fn update_tasks(&self) {
        info!("I will update the task list");
    }

    pub fn send_work(&self) {
        info!("I will send work to the workers");
    }
}
