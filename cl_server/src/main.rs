use std::{io::Cursor, path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use ciborium::from_reader;
use serde::Deserialize;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::sleep,
};

struct Message(Cursor<Vec<u8>>);

impl From<Vec<u8>> for Message {
    fn from(value: Vec<u8>) -> Self {
        Self(Cursor::new(value))
    }
}

impl Message {
    fn into_inner(self) -> Cursor<Vec<u8>> {
        self.0
    }
}

#[derive(Debug, Deserialize)]
enum Command {
    Add(Task),
    Run,
    Kill,
    Show,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Task {
    cmd: String,
    cwd: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct TaskManager {
    // Not sure if a vector is the right type to use, a hashmap could be better
    // since we could create multiple different task categories and get information
    // of a single task by giving the name of the task.
    //
    // We will also need a sender to send information to the processing function.
    // This will be implemented later
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn add_task(&mut self, task: Task) {
        let mut tasks = self.tasks.lock().await;
        println!("Adding task: {:?}", task);
        tasks.push(task);
    }

    async fn execute_tasks(&mut self) {
        let wait = Duration::from_secs(2);
        let mut tasks = self.tasks.lock().await;
        println!("Running all tasks");

        while let Some(task) = tasks.pop() {
            println!("Executing task: {:?}", task);
            sleep(wait).await;
        }

        println!("All tasks have been executed");
    }

    async fn show_all_tasks(&self) {
        let tasks = self.tasks.lock().await;
        println!("{:?}", tasks);
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").await?;
    let task_manager = TaskManager::new();

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .context("Could not get the client")?;
        let task_manager = task_manager.clone();

        tokio::spawn(async move {
            process_connection(socket, task_manager).await.unwrap();
        });
    }
}

async fn process_connection(socket: TcpStream, mut task_manager: TaskManager) -> Result<()> {
    let task = receive_task(socket).await?;

    match task {
        Command::Add(task) => {
            task_manager.add_task(task).await;
        }
        Command::Run => {
            task_manager.execute_tasks().await;
        }
        Command::Kill => {
            // This still needs to be mocked
            println!("Killing task");
        }
        Command::Show => {
            task_manager.show_all_tasks().await;
        }
    }

    Ok(())
}

async fn receive_task(stream: TcpStream) -> Result<Command> {
    let msg = read_bytes(stream).await?;
    let cmd = from_reader::<Command, _>(msg.into_inner()).context("could not deserialize task")?;

    Ok(cmd)
}

async fn read_bytes(mut stream: TcpStream) -> Result<Message> {
    // This could be improved by receiving the size of the payload so that we
    // can initialize an array with the right size instead of initializing a
    // vector
    let mut buf = vec![];
    stream.read_to_end(&mut buf).await?;

    Ok(Message::from(buf))
}
