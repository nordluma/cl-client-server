use std::{collections::VecDeque, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use tokio::{sync::Mutex, task::JoinHandle, time::sleep};

use cl_lib::{
    message::{Message, Payload, Response},
    network::{init_listener, receive_message, send_response, GenericStream},
};
use cl_server::task::Task;

#[derive(Debug, Clone)]
struct TaskManager {
    // We will also need a sender to send information to the processing function.
    // This will be implemented later
    tasks: Arc<Mutex<VecDeque<Task>>>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    async fn add_task(&mut self, task: Payload) {
        let task = task.into();
        let mut tasks = self.tasks.lock().await;
        println!("Adding task: {:?}", task);
        tasks.push_back(task);
    }

    async fn execute_tasks(&mut self) {
        let mut tasks = self.tasks.lock().await;
        let wait = Duration::from_secs(2);
        println!("Running all tasks");

        while let Some(task) = tasks.pop_front() {
            println!("Executing task: {:?}", task);
            sleep(wait).await;
        }

        println!("All tasks have been executed");
    }

    async fn show_all_tasks(&self) {
        let tasks = self.tasks.lock().await;
        if tasks.is_empty() {
            println!("No tasks added");
            return;
        }

        for task in tasks.iter() {
            println!("{:?}", task);
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let listener = init_listener(("127.0.0.1", "42069")).await?;
    let task_manager = TaskManager::new();

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .context("Could not get the client")?;
        let mut task_manager = task_manager.clone();
        let mut stream = Box::new(socket);

        let _: JoinHandle<Result<()>> = tokio::spawn(async move {
            match process_connection(&mut stream, &mut task_manager).await? {
                Some(res) => send_response(res, &mut stream).await?,
                None => send_response(Response::EmptyResponse, &mut stream).await?,
            }

            Ok(())
        });
    }
}

async fn process_connection(
    stream: &mut GenericStream,
    task_manager: &mut TaskManager,
) -> Result<Option<Response>> {
    let task = receive_message(stream).await?;

    let res = match task {
        Message::Add(task) => {
            task_manager.add_task(task).await;
            Some(Response::Success("task added successfully.".to_string()))
        }
        Message::Run => {
            task_manager.execute_tasks().await;
            Some(Response::Success("tasks completed".to_string()))
        }
        Message::Kill => {
            // This still needs to be mocked
            println!("Killing task");
            None
        }
        Message::Show => {
            task_manager.show_all_tasks().await;
            None
        }
    };

    Ok(res)
}
