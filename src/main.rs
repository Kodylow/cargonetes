use manager::Manager;
use node::Node;
use task::{State, Task, TaskEvent};
use tokio::time::Instant;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};
use uuid::Uuid;
use worker::Worker;

mod manager;
mod node;
mod scheduler;
mod task;
mod utils;
mod worker;

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    init_tracing_subscriber()?;

    let task = Task::new(
        Uuid::new_v4(),
        "Task-1".to_string(),
        State::Pending,
        "ubuntu:latest".to_string(),
        1024,
        1,
    );
    info!("Task: {:?}", task);

    let task_event = TaskEvent::new(Uuid::new_v4(), State::Pending, Instant::now(), task);
    info!("Task event: {:?}", task_event);

    let worker = Worker::new("worker-1".to_string());
    info!("Worker: {:?}", worker);

    worker.collect_stats();
    worker.run_task();
    worker.start_task();
    worker.stop_task();

    let manager = Manager::new();
    info!("Manager: {:?}", manager);

    manager.select_worker();
    manager.update_tasks();
    manager.send_work();

    let node = Node::new(
        "node-1".to_string(),
        "192.168.1.1".to_string(),
        4,
        1024,
        25,
        "worker".to_string(),
    );
    info!("Node: {:?}", node);
    Ok(())
}

pub fn init_tracing_subscriber() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive("info".parse()?)
                .from_env()?,
        )
        .init();
    Ok(())
}
