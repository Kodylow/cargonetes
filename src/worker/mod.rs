use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::task::Task;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worker {
    pub name: String,
    pub queue: VecDeque<Task>,
    pub db: BTreeMap<uuid::Uuid, Task>,
    pub task_count: usize,
}

impl Worker {
    pub fn new(name: String) -> Self {
        Self {
            name,
            queue: VecDeque::new(),
            db: BTreeMap::new(),
            task_count: 0,
        }
    }
    pub fn collect_stats(&self) {
        info!("I will collect stats");
    }

    pub fn run_task(&self) {
        info!("I will start or stop a task");
    }

    pub fn start_task(&self) {
        info!("I will start a task");
    }

    pub fn stop_task(&self) {
        info!("I will stop a task");
    }
}
