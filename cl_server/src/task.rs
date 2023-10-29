use std::path::PathBuf;

use cl_lib::message::Payload;

#[derive(Debug)]
pub enum TaskStatus {
    Enqueued,
    Running,
    Paused,
    Completed,
}

#[derive(Debug)]
pub struct Task {
    pub directory: PathBuf,
    pub command: String,
    pub status: TaskStatus,
    pub response: Option<String>,
}

impl From<Payload> for Task {
    fn from(value: Payload) -> Self {
        Self {
            directory: value.cwd,
            command: value.cmd,
            status: TaskStatus::Enqueued,
            response: None,
        }
    }
}

// This is still just for prototyping, haven't decided if this will be used later.
#[allow(dead_code)]
impl Task {
    fn mark_complete(&mut self) {
        self.status = TaskStatus::Completed;
    }

    fn run(&mut self) {
        self.status = TaskStatus::Paused;
    }

    fn start(&mut self) {
        self.status = TaskStatus::Running;
    }
}
