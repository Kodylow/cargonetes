use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub ip: String,
    pub cores: usize,
    pub memory: usize,
    pub memory_allocated: usize,
    pub disk: usize,
    pub disk_allocated: usize,
    pub role: String,
    pub task_count: usize,
}

impl Node {
    pub fn new(
        name: String,
        ip: String,
        cores: usize,
        memory: usize,
        disk: usize,
        role: String,
    ) -> Self {
        Self {
            name,
            ip,
            cores,
            memory,
            memory_allocated: 0,
            disk,
            disk_allocated: 0,
            role,
            task_count: 0,
        }
    }
}
