use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use shellder::{App, CliLogger, Container, Hookable, Hooks, Logger};

pub struct TaskQueue {
    queue: Arc<Mutex<Vec<String>>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(vec![
                "Clean temp files".into(),
                "Send email report".into(),
                "Backup database".into(),
            ])),
        }
    }

    pub fn run_next(&self) {
        let mut queue = self.queue.lock().unwrap();
        if let Some(task) = queue.pop() {
            CliLogger::success(&format!("Running task: {}", task));
        } else {
            CliLogger::warn("No more tasks in queue.");
        }
    }
    pub fn has_tasks(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        return !queue.is_empty();
    }
}

#[derive(App)]
pub struct TaskApp {
    #[component]
    task_queue: Arc<TaskQueue>,
}

impl Hooks for TaskApp {
    fn startup(&self) {
        CliLogger::log("Task Runner Starting Up...");
    }

    fn run(&self) {
        CliLogger::log("Entering main loop.");
        while self.task_queue.has_tasks() {
            self.task_queue.run_next();
            thread::sleep(Duration::from_secs(5));
        }
    }

    fn cleanup(&self) {
        CliLogger::log("Cleaning up before shutdown...");
    }
}
